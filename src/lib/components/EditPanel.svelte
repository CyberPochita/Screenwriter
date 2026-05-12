<script lang="ts">
  let { onApplyStyle } = $props();

  const tools = [
    { label: 'B', syntax: '**', title: 'Жирный' },
    { label: 'I', syntax: '*', title: 'Курсив' },
    { label: 'H1', syntax: '# ', title: 'Заголовок' },
    { label: '---', syntax: '\n---\n', title: 'Разделитель' }
  ];
</script>

<!-- Обертка на всю высоту экрана по правому краю -->
<div class="fixed inset-y-0 right-0 z-[100] w-64 pointer-events-none overflow-hidden">
  
  <!-- Сама панель. translate-x-[calc(100%-12px)] прячет её, оставляя 12px -->
  <div class="absolute top-1/2 right-0 flex w-52 -translate-y-1/2 translate-x-[calc(100%-12px)] rounded-l-2xl border border-r-0 border-white/10 bg-white/5 shadow-2xl backdrop-blur-xl transition-transform duration-300 ease-out hover:translate-x-0 pointer-events-auto">
    
    <!-- Ярлык (Handle) — теперь он гарантированно у края -->
    <div class="flex w-3 items-center justify-center cursor-pointer border-r border-white/5">
      <span class="rotate-270 font-mono text-[8px] text-paper/30 uppercase tracking-tighter whitespace-nowrap">Markdown</span>
    </div>

    <!-- Контент -->
    <div class="flex flex-1 flex-col gap-4 p-6">
      <h3 class="font-mono text-[10px] uppercase tracking-widest text-paper/50 italic">Инструменты</h3>
      
      <div class="grid grid-cols-2 gap-2">
        {#each tools as tool}
          <button 
            class="h-10 rounded-lg border border-white/10 bg-white/5 font-mono text-xs text-paper hover:bg-white hover:text-black transition-all"
            onclick={() => onApplyStyle(tool.syntax)}
          >
            {tool.label}
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>
