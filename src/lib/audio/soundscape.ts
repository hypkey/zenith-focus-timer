export const SOUND_IDS = ["rain", "cafe", "fireplace", "forest", "ocean", "white_noise"] as const;
export type SoundId = (typeof SOUND_IDS)[number];

const SOUND_PATHS: Record<SoundId, string> = {
	rain: "/sounds/rain.mp3",
	cafe: "/sounds/cafe.mp3",
	fireplace: "/sounds/fireplace.mp3",
	forest: "/sounds/forest.mp3",
	ocean: "/sounds/ocean.mp3",
	white_noise: "/sounds/white_noise.mp3",
};

interface SoundState {
	buffer: AudioBuffer;
	gainNode: GainNode;
	source: AudioBufferSourceNode | null;
	targetVolume: number;
}

export class SoundscapeEngine {
	private ctx: AudioContext | null = null;
	private sounds = new Map<string, SoundState>();

	async loadSound(id: string, url: string): Promise<void> {
		if (!this.ctx) this.ctx = new AudioContext();
		const response = await fetch(url);
		if (!response.ok) throw new Error(`Failed to load ${id}: ${response.status}`);
		const arrayBuffer = await response.arrayBuffer();
		const buffer = await this.ctx.decodeAudioData(arrayBuffer);

		const gainNode = this.ctx.createGain();
		gainNode.gain.value = 0;

		this.sounds.set(id, { buffer, gainNode, source: null, targetVolume: 0 });
	}

	play(id: string): void {
		const state = this.sounds.get(id);
		if (!state || !this.ctx) return;

		if (state.source) {
			state.source.stop();
		}

		const source = this.ctx.createBufferSource();
		source.buffer = state.buffer;
		source.loop = true;
		source.connect(state.gainNode);
		state.gainNode.connect(this.ctx.destination);
		state.gainNode.gain.value = state.targetVolume;
		source.start(0);
		state.source = source;
	}

	stop(id: string): void {
		const state = this.sounds.get(id);
		if (!state?.source) return;
		state.source.stop();
		state.source = null;
	}

	setVolume(id: string, volume: number): void {
		const state = this.sounds.get(id);
		if (!state) return;
		const v = Math.max(0, Math.min(1, volume));
		state.targetVolume = v;
		state.gainNode.gain.value = v;
	}

	getVolume(id: string): number {
		const state = this.sounds.get(id);
		return state?.gainNode.gain.value ?? 0;
	}

	isPlaying(id: string): boolean {
		return this.sounds.get(id)?.source != null;
	}

	fadeIn(durationMs: number): void {
		if (!this.ctx) return;
		const start = this.ctx.currentTime;
		const end = start + durationMs / 1000;
		for (const [, state] of this.sounds) {
			if (state.source) {
				state.gainNode.gain.setValueAtTime(0, start);
				state.gainNode.gain.linearRampToValueAtTime(state.targetVolume, end);
			}
		}
	}

	fadeOut(durationMs: number): void {
		if (!this.ctx) return;
		const start = this.ctx.currentTime;
		const end = start + durationMs / 1000;
		for (const [, state] of this.sounds) {
			state.gainNode.gain.setValueAtTime(state.gainNode.gain.value, start);
			state.gainNode.gain.linearRampToValueAtTime(0, end);
		}
	}

	stopAll(): void {
		for (const id of this.sounds.keys()) {
			this.stop(id);
		}
	}

	async loadAll(basePath = ""): Promise<void> {
		const loadPromises: Promise<void>[] = [];
		for (const id of SOUND_IDS) {
			const url = basePath ? `${basePath}${SOUND_PATHS[id]}` : SOUND_PATHS[id];
			loadPromises.push(this.loadSound(id, url).catch(() => {}));
		}
		await Promise.all(loadPromises);
	}

	get loadedIds(): string[] {
		return Array.from(this.sounds.keys());
	}
}
