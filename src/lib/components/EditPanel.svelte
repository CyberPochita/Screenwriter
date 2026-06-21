<script lang="ts">
  import { getContext } from "svelte";
  
  let doc = getContext<any>("doc");

  function generatePerfectWordStyleTitlePage() {
    if (!doc || !doc.currentPage) return;

    const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
    if (!editorDiv) return;
    
    editorDiv.focus();
    editorDiv.innerText = ""; // Чистим лист перед вставкой

    const leftMargin = "\u00A0".repeat(13); // 3.25 см
    const contactMargin = "\u00A0".repeat(52); // 8.25 см

    let textLayout = "";

    textLayout += "\n".repeat(14);
    textLayout += leftMargin + "MAМA НЕ ГОРЮЙ\n\n";
    textLayout += leftMargin + "Константин Мурзенко, Максим Пежемский\n\n";
    textLayout += leftMargin + "Оригинальный сценарий\n";
    textLayout += "\n".repeat(14);
    textLayout += contactMargin + "Константин Мурзенко,\n";
    textLayout += contactMargin + "Максим Пежемский\n";
    textLayout += contactMargin + "http://www.ezhe.ru/data/v\n";
    textLayout += contactMargin + "gik/pm-mama.html";

    // Нативно вставляем текст. Рекурсии больше нет, так как эффект заблокирован!
    document.execCommand("insertText", false, textLayout);

    // Фиксируем итоговое текстовое состояние в реактивном сторе документов
    doc.currentPage.text = editorDiv.innerText;
  }

  function clearCurrentPageText() {
    if (!doc || !doc.currentPage) return;
    doc.currentPage.text = "";
    const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
    if (editorDiv) {
      editorDiv.innerHTML = "";
      editorDiv.dispatchEvent(new Event('input', { bubbles: true }));
    }
  }
</script>

<div class="fixed inset-y-0 right-0 z-50 w-64 pointer-events-none overflow-hidden select-none">
  <div class="absolute top-1/2 right-0 flex w-52 -translate-y-1/2 translate-x-[calc(100%-12px)] rounded-l-2xl border border-r-0 border-black/10 bg-white/80 shadow-2xl backdrop-blur-xl transition-transform duration-300 ease-out hover:translate-x-0 pointer-events-auto">
    
    <div class="flex w-3 items-center justify-center cursor-pointer border-r border-black/5 bg-gray-50/50 rounded-l-2xl">
      <span class="rotate-270 font-mono text-[9px] text-black/30 uppercase tracking-widest whitespace-nowrap">///</span>
    </div>

    <div class="flex flex-1 flex-col gap-4 p-5 font-sans">
      <h3 class="font-mono text-[10px] uppercase tracking-widest text-black/40 font-bold border-b border-black/5 pb-1">
        Макросы
      </h3>

      <div class="flex flex-col gap-2">
        <button
          onclick={generatePerfectWordStyleTitlePage}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-black text-white font-mono text-xs transition-all hover:bg-gray-800 active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>✨ Макет титула</span>
          <span class="opacity-50">📄</span>
        </button>

        <button
          onclick={clearCurrentPageText}
          class="w-full h-9 text-left px-3 rounded-lg border border-gray-200 bg-white text-gray-600 font-mono text-xs transition-all hover:bg-red-50 hover:text-red-600 hover:border-red-200 active:scale-[0.98] outline-none"
        >
          🗑️ Очистить лист
        </button>
      </div>
    </div>

  </div>
</div>
