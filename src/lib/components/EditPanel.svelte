<script lang="ts">
  import { getContext } from "svelte";

  let doc = getContext<any>("doc");

  function generatePerfectWordStyleTitlePage() {
    if (!doc || !doc.currentPage) return;

    const editorDiv = document.querySelector(
      '[role="textbox"]',
    ) as HTMLDivElement;
    if (!editorDiv) return;

    editorDiv.focus();
    editorDiv.innerHTML = ""; // Полностью очищаем лист перед вставкой

    // Использование &nbsp; гарантирует сохранение полей в памяти Svelte при смене страниц
    const leftMargin = "&nbsp;".repeat(13); // 3.25 см
    const contactMargin = "&nbsp;".repeat(52); // 8.25 см

    let htmlLayout = "";

    // 1. Отступ сверху — 14 пустых строк
    for (let i = 0; i < 14; i++) {
      htmlLayout += "<div><br></div>";
    }

    // 2. Название сценария ЗАГЛАВНЫМИ
    htmlLayout += `<div>${leftMargin}МАМА НЕ ГОРЮЙ</div>`;
    htmlLayout += "<div><br></div>";

    // 3. Имя автора
    htmlLayout += `<div>${leftMargin}Константин Мурзенко, Максим Пежемский</div>`;
    htmlLayout += "<div><br></div>";

    // 4. Указание авторства
    htmlLayout += `<div>${leftMargin}Оригинальный сценарий</div>`;

    // 5. Сдвиг до низа страницы (16 пустых строк)
    for (let i = 0; i < 16; i++) {
      htmlLayout += "<div><br></div>";
    }

    // 6. Блок контактов
    htmlLayout += `<div>${contactMargin}Константин Мурзенко,</div>`;
    htmlLayout += `<div>${contactMargin}Максим Пежемский</div>`;
    htmlLayout += `<div>${contactMargin}http://www.ezhe.ru/data/v</div>`;
    htmlLayout += `<div>${contactMargin}gik/pm-mama.html</div>`;

    // Вставляем HTML. Команда отработает идеально, без рекурсии
    document.execCommand("insertHTML", false, htmlLayout);

    // Записываем HTML-код в реактивный стор текущей страницы для Rust XML
    doc.currentPage.text = editorDiv.innerHTML;
  }

  function selectElementText(element: HTMLElement) {
    if (!element) return;

    // Находим текстовый узел внутри элемента (чтобы не зацепить теги)
    const textNode = element.firstChild;
    if (!textNode) return;

    const range = document.createRange();
    const selection = window.getSelection();

    if (selection) {
      // Если текст начинается с неразрывных пробелов (&nbsp;),
      // выделяем только сам текст, пропуская отступы полей
      const rawText = textNode.textContent || "";
      const firstCharIndex = rawText.search(/[^\u00A0\s]/); // Находим индекс первой буквы

      const startOffset = firstCharIndex !== -1 ? firstCharIndex : 0;
      const endOffset = rawText.length;

      range.setStart(textNode, startOffset);
      range.setEnd(textNode, endOffset);

      selection.removeAllRanges();
      selection.addRange(range);
    }
  }

  /**
   * Вспомогательный метод для вставки HTML и автоматического выделения нужной строки
   */
  function insertAndSelect(htmlContent: string, targetSelector: string) {
    if (!doc || !doc.currentPage) return;

    const editorDiv = document.querySelector(
      '[role="textbox"]',
    ) as HTMLDivElement;
    if (!editorDiv) return;

    editorDiv.focus();

    // Вставляем HTML структуру через команду процессора
    document.execCommand("insertHTML", false, htmlContent);

    // Даем браузеру микросекунду на отрисовку DOM, находим элемент и выделяем его текст
    setTimeout(() => {
      const insertedElements = editorDiv.querySelectorAll(targetSelector);
      if (insertedElements.length > 0) {
        // Берем самый последний вставленный элемент этого типа
        const lastElement = insertedElements[
          insertedElements.length - 1
        ] as HTMLElement;
        selectElementText(lastElement);

        // Убираем временный класс, если он использовался для поиска
        if (lastElement.classList.contains("temp-select-target")) {
          lastElement.classList.remove("temp-select-target");
        }
      }
      // Синхронизируем итоговый HTML со стором Svelte
      doc.currentPage.text = editorDiv.innerHTML;
    }, 10);
  }

  // 1. Макрос: Время и место действия (Отступ 3.75 см = 15 пробелов)
  function insertSceneHeadingLayout() {
    const leftMargin = "&nbsp;".repeat(15);
    const text = "НАТ. ДВОР СТАНДАРТНОГО ДОМА - ДЕНЬ";
    const html = `<div><br></div><div class="temp-select-target">${leftMargin}${text}</div><div><br></div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  // 2. Макрос: Описание действия (0 дополнительных пробелов)
  function insertActionDescriptionLayout() {
    const text =
      "МАЙОР мрачно ест пельмени. Пьет пиво. На столе перед ним красная клеенчатая папка.";
    const html = `<div><br></div><div class="temp-select-target">${text}</div><div><br></div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  // 3. Макрос: Имя героя (Отступ 10.5 см = 29 пробелов внутри строки)
  function insertCharacterNameLayout() {
    const leftMargin = "&nbsp;".repeat(29);
    const text = "СЛЕДОВАТЕЛЬ";
    const html = `<div><br></div><div class="temp-select-target">${leftMargin}${text}</div>`;
    insertAndSelect(html, ".temp-select-target");
  }

  // 4. Макрос: Реплика героя (CSS-отступы 7.5 см слева, 6.25 см справа)
  function insertCharacterDialogueLayout() {
    const text = "Ты лучше бы не каркал...";
    const html = `<div class="script-dialogue">${text}</div><div><br></div>`;
    insertAndSelect(html, ".script-dialogue");
  }

  // 5. Макрос: Ремарка (CSS-отступы 9.25 см слева, 6.25 см справа)
  function insertParentheticalLayout() {
    const text = "(рассматривает пистолет)";
    const html = `<div class="script-parenthetical">${text}</div>`;
    insertAndSelect(html, ".script-parenthetical");
  }

  // 6. Макрос: Титр сценария (Вариант 3)
  function insertTitleLayout() {
    const text = "Квартира Зубека, 12 июля, 12:57";
    const html = `<div>ТИТР:</div><div><br></div><div class="script-title-text">${text}</div><div><br></div>`;
    insertAndSelect(html, ".script-title-text");
  }

  // Старый макрос полного заполнения Титульного листа (Word-style)
  function generatePerfectXMLTitlePage() {
    if (!doc) return;
    doc.titlePage = {
      formatting: {
        top_margin: 14,
        left_margin: 3.25,
        right_margin: 3.25,
        contact_left_margin: 8.25,
      },
      title: "МАМА НЕ ГОРЮЙ",
      author: "Константин Мурзенко, Максим Пежемский",
      authorship: "original",
      contact: {
        name: "Константин Мурзенко",
        address: "http://ezhe.ru",
        phone: "gik/pm-mama.html",
        email: null,
        agent: null,
      },
    };
    doc.activeTab = "title";
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

<!-- Код разметки панели остается без изменений -->
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
