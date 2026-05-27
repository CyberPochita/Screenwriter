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

  let isHeaderMenuOpen = $state(false);

  function selectHeader(syntax: string) {
    onApplyStyle(syntax);
    isHeaderMenuOpen = false;
  }
</script>

<div
  class="fixed inset-y-0 right-0 z-100 w-64 pointer-events-none overflow-hidden"
>
  <div
    class="absolute top-1/2 right-0 flex w-52 -translate-y-1/2 translate-x-[calc(100%-12px)] rounded-l-2xl border border-r-0 border-white/10 bg-white/5 shadow-2xl backdrop-blur-xl transition-transform duration-300 ease-out hover:translate-x-0 pointer-events-auto"
  >
    <div
      class="flex w-3 items-center justify-center cursor-pointer border-r border-white/5"
    >
      <span
        class="rotate-270 font-mono text-[10px] text-paper/30 uppercase tracking-tighter whitespace-nowrap"
        >===</span
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
      </div>
    </div>
  </div>
</div>
