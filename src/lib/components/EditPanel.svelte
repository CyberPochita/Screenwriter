<script lang="ts">
  import Layout from "../../routes/+layout.svelte";

  let { onApplyStyle } = $props();

  const tools = [
    { label: "B", syntax: "**", title: "Жирный" },
    { label: "I", syntax: "*", title: "Курсив" },
    { label: "BI", syntax: "***", title: "Жирный курсив" },
    { label: "S", syntax: "~~", title: "Зачеркнутый" },
    { label: "Code", syntax: "`", title: "Моноширинный" },
    { label: "Quote", syntax: "> ", title: "Блок цитирования" },
    { label: "---", syntax: "--- ", title: "Разделитель" },
  ];

  const headers = [
    { label: "H1", syntax: "# ", title: "Заголовок" },
    { label: "H2", syntax: "## ", title: "Подзаголовок" },
    { label: "H3", syntax: "### ", title: "Подподзаголовок" },
    { label: "H4", syntax: "#### ", title: "Подподзаголовок" },
    { label: "H5", syntax: "##### ", title: "Подподзаголовок" },
    { label: "H6", syntax: "###### ", title: "Подподзаголовок" },
  ];

  let isHeaderMenuOpen = $state(false);

  function selectHeader(syntax: string) {
    onApplyStyle(syntax);
    isHeaderMenuOpen = false;
  }
</script>

<!-- Обертка на всю высоту экрана по правому краю -->
<div
  class="fixed inset-y-0 right-0 z-[100] w-64 pointer-events-none overflow-hidden"
>
  <!-- Сама панель. translate-x-[calc(100%-12px)] прячет её, оставляя 12px -->
  <div
    class="absolute top-1/2 right-0 flex w-52 -translate-y-1/2 translate-x-[calc(100%-12px)] rounded-l-2xl border border-r-0 border-white/10 bg-white/5 shadow-2xl backdrop-blur-xl transition-transform duration-300 ease-out hover:translate-x-0 pointer-events-auto"
  >
    <!-- Ярлык (Handle) — теперь он гарантированно у края -->
    <div
      class="flex w-3 items-center justify-center cursor-pointer border-r border-white/5"
    >
      <span
        class="rotate-270 font-mono text-[8px] text-paper/30 uppercase tracking-tighter whitespace-nowrap"
        >Markdown</span
      >
    </div>

    <!-- Контент -->
    <div class="flex flex-1 flex-col gap-4 p-6">
      <h3
        class="font-mono text-[10px] uppercase tracking-widest text-paper/50 italic"
      >
        Инструменты
      </h3>

      <div class="grid grid-cols-2 gap-2">
        {#each tools as tool}
          <button 
            class="h-10 rounded-lg border border-white/10 bg-white/5 font-mono text-xs text-paper transition-all hover:bg-white hover:text-black outline-none" 
            onclick={() => onApplyStyle(tool.syntax)}
          >
            {tool.label}
          </button>
        {/each}
        <!-- Выпадающее меню заголовков -->
        <div class="relative col-span-2">
          <button 
            class="flex h-10 w-full items-center justify-between rounded-lg border border-white/10 px-4 font-mono text-xs text-paper transition-all outline-none hover:bg-white/10 {isHeaderMenuOpen ? 'bg-white !text-black' : 'bg-white/5'}"
            onclick={() => isHeaderMenuOpen = !isHeaderMenuOpen}
          >
            <span>Заголовки</span>
            <span class="text-[8px] opacity-50">{isHeaderMenuOpen ? '▲' : '▼'}</span>
          </button>

          {#if isHeaderMenuOpen}
            <div
              class="absolute bottom-full left-0 mb-2 w-full flex flex-col rounded-lg bg-[#1a1a1a]/95 backdrop-blur-md shadow-2xl overflow-hidden z-110"
            >
              {#each headers as h}
                <button
                  class="flex items-center gap-3 px-4 py-2 font-mono text-[10px] text-paper bg-white text-black hover:bg-black hover:text-white transition-all text-left border-b border-white/5 last:border-0"
                  onclick={() => selectHeader(h.syntax)}
                >
                  <span class="opacity-30">{h.label}</span>
                  <span class="truncate text-xs">Заголовок {h.label.slice(1)}</span>
                </button>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
