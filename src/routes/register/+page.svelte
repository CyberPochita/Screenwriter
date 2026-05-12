<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  let username = $state('');
  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let isLoading = $state(false);
</script>

<div class="flex min-h-screen items-center justify-center p-6">
  <div class="w-full max-w-lg rounded-sm bg-white/75 p-10">
    
    <header class="flex mb-10 justify-between items-center">
      <h1 class="font-mono text-xs uppercase tracking-[0.3em] text-editor-blue">Регистрация</h1>
      <h1 class="font-mono text-xs uppercase tracking-[0.3em] text-editor-blue">Screenwriter</h1>
    </header>

    <form class="space-y-8">
      <!-- {#if $authError}
        <div class="border border-red-500/50 bg-red-500/10 p-3 font-mono text-xs text-red-400">
          > ОШИБКА: {$authError}
        </div>
      {/if} -->

      <div class="group relative">
        <label for="user" class="font-mono text-[12px] uppercase text-paper/40 group-focus-within:text-editor-blue transition-colors">Автор</label>
        <input id="user" type="text" bind:value={username} required placeholder="..."
          class="w-full border-b border-paper/20 bg-transparent py-2 font-serif text-xl text-paper outline-none focus:border-editor-blue transition-all placeholder:text-paper/10" />
      </div>

      <div class="group relative">
        <label for="email" class="font-mono text-[12px] uppercase text-paper/40 group-focus-within:text-editor-blue transition-colors">Email</label>
        <input id="email" type="email" bind:value={email} required placeholder="..."
          class="w-full border-b border-paper/20 bg-transparent py-2 font-serif text-xl text-paper outline-none focus:border-editor-blue transition-all placeholder:text-paper/10" />
      </div>

      <div class="grid grid-cols-2 gap-8">
        <div class="group relative">
          <label for="pass" class="font-mono text-[12px] uppercase text-paper/40 group-focus-within:text-editor-blue transition-colors">Пароль</label>
          <input id="pass" type="password" bind:value={password} required placeholder="..."
            class="w-full border-b border-paper/20 bg-transparent py-2 text-paper text-xl outline-none focus:border-editor-blue transition-all placeholder:text-paper/10" />
        </div>
        <div class="group relative">
          <label for="conf" class="font-mono text-[12px] uppercase text-paper/40 group-focus-within:text-editor-blue transition-colors">Повтор пароля</label>
          <input id="conf" type="password" bind:value={confirmPassword} required placeholder="..."
            class="w-full border-b border-paper/20 bg-transparent py-2 text-paper text-xl outline-none focus:border-editor-blue transition-all placeholder:text-paper/10" />
        </div>
      </div>

      <div class="pt-4">
        <!-- <button type="submit" disabled={isLoading}
          class="group relative w-full overflow-hidden border border-paper/20 bg-transparent py-4 font-mono text-xs uppercase tracking-widest text-paper hover:bg-white hover:text-writer-dark hover:shadow-[0_0_20px_rgba(255,255,255,0.15)] disabled:opacity-20">
          <span class="relative z-10">{isLoading ? 'Идет запись...' : 'Начать работу'}</span>
        </button> -->
        <button type="submit" disabled={isLoading}
          class="group relative w-full border border-white/20 bg-transparent py-4 font-mono text-xs uppercase tracking-[0.2em] text-paper transition-all hover:bg-white hover:text-writer-dark hover:shadow-[0_0_20px_rgba(255,255,255,0.15)] disabled:opacity-20">
          {isLoading ? 'Идет запись...' : 'Начать работу'}
        </button>
      </div>
    </form>

    <footer class="mt-12 flex justify-center font-mono text-[13px] text-paper/30">
      <a href="/" class="hover:text-editor-blue transition-colors underline decoration-dotted">Вернуться ко входу</a>
    </footer>
  </div>
</div>

<style>
  /* Имитация шрифта печатной машинки для заголовков, если есть в системе */
  :global(body) {
    cursor: default;
  }
</style>
