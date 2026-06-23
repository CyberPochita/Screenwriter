<script lang="ts">
  import { getContext } from "svelte";
  import "$lib/../theme.css"

  let doc = getContext<any>("doc");

  function generatePerfectWordStyleTitlePage() {
    if (!doc || !doc.currentPage) return;
    const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
    if (!editorDiv) return;

    editorDiv.focus();
    editorDiv.innerHTML = "";

    const leftMargin = "&nbsp;".repeat(13);
    const contactMargin = "&nbsp;".repeat(52);
    let htmlLayout = "";

    for (let i = 0; i < 14; i++) htmlLayout += "<div><br></div>";
    htmlLayout += `<div>${leftMargin}МАМА НЕ ГОРЮЙ</div><div><br></div>`;
    htmlLayout += `<div>${leftMargin}Константин Мурзенко, Максим Пежемский</div><div><br></div>`;
    htmlLayout += `<div>${leftMargin}Оригинальный сценарий</div>`;
    for (let i = 0; i < 16; i++) htmlLayout += "<div><br></div>";
    htmlLayout += `<div>${contactMargin}Константин Мурзенко,</div>`;
    htmlLayout += `<div>${contactMargin}Максим Пежемский</div>`;
    htmlLayout += `<div>${contactMargin}http://ezhe.ru</div>`;
    htmlLayout += `<div>${contactMargin}gik/pm-mama.html</div>`;

    document.execCommand("insertHTML", false, htmlLayout);
    doc.currentPage.text = editorDiv.innerHTML;
  }

  function selectElementText(element: HTMLElement) {
    if (!element) return;
    const textNode = element.firstChild;
    if (!textNode) return;
    const range = document.createRange();
    const selection = window.getSelection();

    if (selection) {
      const rawText = textNode.textContent || "";
      const firstCharIndex = rawText.search(/[^\u00A0\s]/);
      range.setStart(textNode, firstCharIndex !== -1 ? firstCharIndex : 0);
      range.setEnd(textNode, rawText.length);
      selection.removeAllRanges();
      selection.addRange(range);
    }
  }

  function insertAndSelect(htmlContent: string, targetSelector: string) {
    if (!doc || !doc.currentPage) return;
    const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
    if (!editorDiv) return;

    editorDiv.focus();
    document.execCommand("insertHTML", false, htmlContent);

    setTimeout(() => {
      const insertedElements = editorDiv.querySelectorAll(targetSelector);
      if (insertedElements.length > 0) {
        const lastElement = insertedElements[insertedElements.length - 1] as HTMLElement;
        selectElementText(lastElement);
        if (lastElement.classList.contains("temp-select-target")) {
          lastElement.classList.remove("temp-select-target");
        }
      }
      doc.currentPage.text = editorDiv.innerHTML;
    }, 10);
  }

  function insertSceneHeadingLayout() {
    const html = `<div><br></div><div class="temp-select-target font-courier text-12pt uppercase tracking-wide">НАТ. ДВОР СТАНДАРТНОГО ДОМА - ДЕНЬ</div><div><br></div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  function insertActionDescriptionLayout() {
    const html = `<div><br></div><div class="temp-select-target font-courier text-12pt  text-justify">МАЙОР мрачно ест пельмени. Пьет пиво. На столе перед ним красная клеенчатая папка.</div><div><br></div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  function insertCharacterNameLayout() {
    const leftMargin = "&nbsp;".repeat(35);
    const html = `<div><br></div><div class="temp-select-target font-courier text-12pt  uppercase">${leftMargin}СЛЕДОВАТЕЛЬ</div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  function insertCharacterDialogueLayout() {
    const html = `<div class="script-dialogue font-courier text-12pt">Ты лучше бы не каркал...</div><div><br></div>`;
    insertAndSelect(html, ".script-dialogue");
  }

  function insertParentheticalLayout() {
    const html = `<div class="script-parenthetical font-courier text-12pt ">(рассматривает пистолет)</div>`;
    insertAndSelect(html, ".script-parenthetical");
  }

  function insertTitleLayout() {
    const html = `<div>ТИТР:</div><div><br></div><div class="script-title-text font-courier text-12pt ">Квартира Зубека, 12 июля, 12:57</div><div><br></div>`;
    insertAndSelect(html, ".script-title-text");
  }

  function clearCurrentPageText() {
    if (!doc || !doc.currentPage) return;
    doc.currentPage.text = "";
    const editorDiv = document.querySelector('[role="textbox"]');
    if (editorDiv) {
      editorDiv.innerHTML = "<div><br></div>";
      editorDiv.dispatchEvent(new Event("input", { bubbles: true }));
    }
  }
</script>

<div
  class="fixed inset-y-0 right-0 z-50 w-64 pointer-events-none overflow-hidden select-none"
>
  <div
    class="absolute top-1/2 right-0 flex w-52 -translate-y-1/2 translate-x-[calc(100%-12px)] rounded-l-2xl border border-r-0 border-black/10 bg-white/80 shadow-2xl backdrop-blur-xl transition-transform duration-300 ease-out hover:translate-x-0 pointer-events-auto"
  >
    <div
      class="flex w-3 items-center justify-center cursor-pointer border-r border-black/5 bg-gray-50/50 rounded-l-2xl"
    >
      <span
        class="rotate-270 font-mono text-[9px] text-black/30 uppercase tracking-widest whitespace-nowrap"
        >///</span
      >
    </div>
    <div class="flex flex-1 flex-col gap-4 p-5 font-sans">
      <h3
        class="font-mono text-[12px] uppercase tracking-widest text-black/40 font-bold border-b border-black/5 pb-1"
      >
        Панель управления
      </h3>
      <div class="flex flex-col gap-2">
        <button
          onclick={generatePerfectWordStyleTitlePage}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Макет титула</span>
        </button>
        <button
          onclick={insertSceneHeadingLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Время и место</span>
        </button>
        <button
          onclick={insertActionDescriptionLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Описание действия</span>
        </button>
        <button
          onclick={insertCharacterNameLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Имя героя</span>
        </button>
        <button
          onclick={insertCharacterDialogueLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Реплика героя</span>
        </button>
        <button
          onclick={insertParentheticalLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Ремарка</span>
        </button>
        <button
          onclick={insertTitleLayout}
          class="w-full h-11 text-left px-3 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] transition-all hover:bg-black hover:text-white active:scale-[0.98] outline-none flex items-center justify-between"
        >
          <span>Титр</span>
        </button>
        <div class="border-t border-black/5 my-1"></div>
        <button
          onclick={clearCurrentPageText}
          class="w-full h-9 text-left px-3 rounded-lg border border-gray-200 bg-white text-gray-600 font-mono text-xs transition-all hover:bg-red-50 hover:text-red-600 hover:border-red-200 active:scale-[0.98] outline-none"
        >
          Очистить лист
        </button>
      </div>
    </div>
  </div>
</div>
