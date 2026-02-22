<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Button } from '$lib/components/ui/button';

  let greeting = $state('');

  async function greet() {
    greeting = await invoke<string>('greet', { name: 'Zenith' });
  }
</script>

<main class="flex flex-col items-center justify-center h-screen gap-4">
  <h1 class="text-4xl font-semibold text-foreground">Zenith</h1>
  <p class="text-muted-foreground">Focus timer — coming soon</p>
  <Button onclick={greet}>Test Rust IPC</Button>
  <Button variant="outline" onclick={() => greeting = ''}>Clear</Button>
  {#if greeting}
    <p class="text-accent font-medium">{greeting}</p>
  {/if}
</main>
