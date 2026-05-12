<script lang="ts">
</script>

<div class="flex min-h-full items-center justify-center p-8">
  <div class="w-full max-w-96 rounded-2xl border border-white/10 bg-white/75 p-10 text-center shadow-2xl backdrop-blur-3xl">
    
    <header class="mb-10">
      <h1 class="font-mono text-[10px] tracking-[0.3em] uppercase text-paper/40">
        Screenwriter
      </h1>
      <h2 class="mt-2 font-serif text-4xl italic text-writer-focus">
        Терминал автора
      </h2>
      <p class="mt-4 text-sm text-paper/50">
        Для работы с сценариями войдите в свой профиль или создайте новый
      </p>
    </header>

    <div class="flex flex-col gap-4">
      <!-- Кнопка Входа -->
      <a href="/login" 
         class="flex flex-col items-center justify-center rounded-xl bg-transparent p-6 text-paper transition-all hover:border-writer-focus hover:bg-white/50 hover:text-writer-focus">
        <span class="font-mono text-sm font-bold uppercase tracking-widest">Войти</span>
        <span class="mt-1 text-[10px] opacity-50 text-center">Войти в свой профиль</span>
      </a>

      <div class="my-2 flex items-center gap-4">
        <div class="h-px flex-1 bg-black/25"></div>
        <span class="font-mono text-xs uppercase">ИЛИ</span>
        <div class="h-px flex-1 bg-black/25"></div>
      </div>

      <!-- Кнопка Регистрации -->
      <a href="/register" 
         class="flex flex-col items-center justify-center rounded-xl bg-transparent p-6 text-paper transition-all hover:border-writer-focus hover:bg-white/50 hover:text-writer-focus">
        <span class="font-mono text-sm font-bold uppercase tracking-widest">Новая рукопись</span>
        <span class="mt-1 text-[10px] opacity-50 text-center">Зарегистрировать профиль</span>
      </a>
    </div>

    <footer class="mt-10 flex justify-between font-mono text-[9px] tracking-widest">
      <span>V.2024.1.0_STABLE</span>
      <span>SYSTEM_ID: ARM_WRITER</span>
    </footer>
  </div>
</div>
