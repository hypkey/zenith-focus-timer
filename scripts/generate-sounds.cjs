/**
 * Generates simple chime WAV files for session transitions.
 * Run: node scripts/generate-sounds.cjs
 */

const fs = require("fs");
const path = require("path");

const OUT_DIR = path.join(__dirname, "../src-tauri/resources/sounds");
const SAMPLE_RATE = 44100;

function createWav(freq, durationSec, decay) {
  const numSamples = Math.floor(SAMPLE_RATE * durationSec);
  const data = Buffer.alloc(44 + numSamples * 2);
  let offset = 0;

  const write = (buf) => {
    buf.copy(data, offset);
    offset += buf.length;
  };

  const writeU32 = (n) => write(Buffer.from(new Uint32Array([n]).buffer));
  const writeU16 = (n) => write(Buffer.from(new Uint16Array([n]).buffer));

  write(Buffer.from("RIFF", "ascii"));
  writeU32(36 + numSamples * 2);
  write(Buffer.from("WAVE", "ascii"));
  write(Buffer.from("fmt ", "ascii"));
  writeU32(16);
  writeU16(1);
  writeU16(1);
  writeU32(SAMPLE_RATE);
  writeU32(SAMPLE_RATE * 2);
  writeU16(2);
  writeU16(16);
  write(Buffer.from("data", "ascii"));
  writeU32(numSamples * 2);

  for (let i = 0; i < numSamples; i++) {
    const t = i / SAMPLE_RATE;
    const env = Math.exp(-t / decay);
    const sample = Math.sin(2 * Math.PI * freq * t) * env * 0.3 * 32767;
    const s16 = Math.max(-32768, Math.min(32767, Math.round(sample)));
    data.writeInt16LE(s16, offset);
    offset += 2;
  }

  return data;
}

fs.mkdirSync(OUT_DIR, { recursive: true });

const workChime = createWav(880, 0.4, 0.15);
fs.writeFileSync(path.join(OUT_DIR, "work_complete.wav"), workChime);
console.log("Created work_complete.wav");

const breakChime = createWav(660, 0.35, 0.2);
fs.writeFileSync(path.join(OUT_DIR, "break_complete.wav"), breakChime);
console.log("Created break_complete.wav");
