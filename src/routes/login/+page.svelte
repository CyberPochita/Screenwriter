<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let email = $state('');
  let password = $state('');
  let isLoading = $state(false);

  function handleSubmit(event: Event) {
    if(email == 'admin@admin' && password == 'admin') {
      console.log('Success!');
      console.log(window.location);
      window.location.href = '/scenario';
    }
  }

</script>

<div class="flex min-h-screen items-center justify-center p-6">
  <!-- Карточка входа -->
  <div class="w-full max-w-lg rounded-sm bg-white/75 p-10">
    
    <header class="flex mb-10 justify-between items-center">
      <h1 class="font-mono text-xs uppercase tracking-[0.3em] text-editor-blue">Вход</h1>
      <h1 class="font-mono text-xs uppercase tracking-[0.3em] text-editor-blue">Screenwriter</h1>
    </header>

    <form class="space-y-8" onsubmit={handleSubmit}>
      <!-- {#if $authError}
        <div class="border border-white/20 bg-white/5 p-3 font-mono text-[10px] text-white uppercase tracking-tighter">
          > ОШИБКА АВТОРИЗАЦИИ: {$authError}
        </div>
      {/if} -->

      <div class="group relative">
        <label for="email" class="font-mono text-[12px] uppercase text-paper/30 group-focus-within:text-writer-focus transition-colors">Email</label>
        <input id="email" type="email" bind:value={email} required placeholder="..."
          class="w-full border-b border-paper/10 bg-transparent py-2 font-serif text-xl text-paper outline-none focus:border-writer-focus transition-all placeholder:text-paper/5" />
      </div>

      <div class="group relative">
        <label for="pass" class="font-mono text-[12px] uppercase text-paper/30 group-focus-within:text-writer-focus transition-colors">Код доступа</label>
        <input id="pass" type="password" bind:value={password} required placeholder="..."
          class="w-full border-b border-paper/10 bg-transparent py-2 text-paper text-xl outline-none focus:border-writer-focus transition-all" />
      </div>

      <div class="pt-6">
        <button type="submit" disabled={isLoading}
          class="group relative w-full border border-white/20 bg-transparent py-4 font-mono text-[12px] uppercase tracking-[0.2em] text-paper transition-all hover:bg-white hover:text-writer-dark hover:shadow-[0_0_20px_rgba(255,255,255,0.15)] disabled:opacity-20">
          {isLoading ? 'Проверка прав...' : 'Авторизоваться'}
        </button>
      </div>
    </form>

    <div class="mt-8 flex items-center justify-between font-mono text-[11px] tracking-widest text-paper/20 uppercase">
      <a href="/forgot" class="hover:text-writer-focus transition-colors decoration-dotted">Забыли код?</a>
      <a href="/register" class="hover:text-editor-blue transition-colors underline decoration-dotted">Зарегистрироваться</a>
    </div>
  </div>
</div>

<style>
  /* Дополнительные стили для специфических состояний, если нужно */
  input:-webkit-autofill,
  input:-webkit-autofill:hover, 
  input:-webkit-autofill:focus {
    -webkit-text-fill-color: var(--color-paper);
    -webkit-box-shadow: 0 0 0px 1000px transparent inset;
    transition: background-color 5000s ease-in-out 0s;
  }
</style>
