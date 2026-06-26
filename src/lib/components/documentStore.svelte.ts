import { invoke } from '@tauri-apps/api/core';

// Строгие типы блоков для полной синхронизации с Rust Enum ScenarioBlock
export interface IScenarioBlock {
  block_type: "action_description" | "dialog" | "name_character" | "parenthetical" | "scene_heading" | "title";
  text?: string;
  place?: string;
  title?: string;
  time?: string;
  style?: "center" | "left" | "bottom" | null;
}

export interface IPageContent {
  id: number;
  number: number;
  formatting: {
    top_margin: number;
    left_margin: number;
    right_margin: number;
    contact_left_margin: number;
  };
  blocks: IScenarioBlock[]; // Чистые структуры для бэкенда на Rust
  text: string;            // Сырой HTML-код для contenteditable
}

const DEFAULT_SCRIPT_FORMATTING = {
  top_margin: 0,
  left_margin: 3.25,
  right_margin: 2.5,
  contact_left_margin: 0,
};

export function createDocumentStore() {
  // Инициализируем хранилище с одной пустой страницей по умолчанию
  let pages = $state<IPageContent[]>([
    { id: 1, number: 1, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, blocks: [], text: "<div><br></div>" },
  ]);
  let currentIndex = $state(0);
  let showConfirmDelete = $state(false);

  // Вспомогательная функция для автоматического возврата фокуса в contenteditable
  function focusEditor() {
    queueMicrotask(() => {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor) {
        editor.focus();
        // Устанавливаем курсор в самый конец текста
        const range = document.createRange();
        const sel = window.getSelection();
        range.selectNodeContents(editor);
        range.collapse(false);
        sel?.removeAllRanges();
        sel?.addRange(range);
      }
    });
  }

  // КОНВЕРТЕР (XML -> HTML): Собирает из чистых структур Rust HTML-строки с Tailwind классами для contenteditable
  function compileBlocksToHtml(blocks: IScenarioBlock[]): string {
    if (!blocks || blocks.length === 0) return "<div><br></div>";
    
    return blocks.map(block => {
      switch (block.block_type) {
        case "title":
          const align = block.style ? ` style-${block.style}` : " style-center";
          return `<div class="scen-title font-mono text-[12pt] text-center my-6${align}">${block.text || ""}</div>`;
        case "scene_heading":
          return `<div class="scen-heading font-mono text-[12pt] font-bold uppercase tracking-wide mt-6 mb-3"><b>${block.place || "ИНТ."}</b> ${block.title || ""} — ${block.time || "ДЕНЬ"}</div>`;
        case "action_description":
          return `<div class="scen-action font-mono text-[12pt] text-justify my-3">${block.text || ""}</div>`;
        case "name_character":
          return `<div class="scen-character font-mono text-[12pt] uppercase pl-[35%] mt-4 mb-1">${block.text || ""}</div>`;
        case "parenthetical":
          return `<div class="scen-parenthetical font-mono text-[12pt] px-[25%] mb-1">(${block.text || ""})</div>`;
        case "dialog":
          return `<div class="scen-dialog font-mono text-[12pt] px-[20%] mb-3">${block.text || ""}</div>`;
        default:
          return `<div class="font-mono text-[12pt] my-2">${block.text || ""}</div>`;
      }
    }).join("");
  }

  // ПАРСЕР (HTML -> XML): Переводит HTML-разметку с Tailwind маркерами в чистый массив структур для отправки в Rust
  function parseHtmlToBlocks(htmlText: string): IScenarioBlock[] {
    if (!htmlText) return [];
    
    const parser = new DOMParser();
    const doc = parser.parseFromString(htmlText, "text/html");
    const lines = doc.body.children;
    const blocks: IScenarioBlock[] = [];

    for (const line of Array.from(lines)) {
      const textContent = line.textContent?.trim() || "";
      // Полностью отсекаем пустые переводы строк браузера
      if (!textContent || line.innerHTML === "<br>") continue;

      const className = line.className || "";

      // Распознаем типы блоков по их уникальным семантическим префиксам
      if (className.includes("scen-title")) {
        let style: "center" | "left" | "bottom" = "center";
        if (className.includes("style-left")) style = "left";
        if (className.includes("style-bottom")) style = "bottom";
        blocks.push({ block_type: "title", text: textContent, style });
      } 
      else if (className.includes("scen-heading")) {
        const placeElement = line.querySelector("b");
        const place = placeElement?.textContent?.trim() || "ИНТ.";
        // Вырезаем место из общего текста строки, чтобы получить оставшееся описание и время
        const remainingText = textContent.replace(place, "").trim();
        const parts = remainingText.split("—");
        const title = parts[0]?.trim() || "СЦЕНА";
        const time = parts[1]?.trim() || "ДЕНЬ";
        blocks.push({ block_type: "scene_heading", place, title, time });
      } 
      else if (className.includes("scen-action")) {
        blocks.push({ block_type: "action_description", text: textContent });
      } 
      else if (className.includes("scen-character")) {
        blocks.push({ block_type: "name_character", text: textContent });
      } 
      else if (className.includes("scen-parenthetical")) {
        // Очищаем ремарку от визуальных скобок перед отправкой в БД/XML
        const cleanText = textContent.replace(/[()]/g, "");
        blocks.push({ block_type: "parenthetical", text: cleanText });
      } 
      else if (className.includes("scen-dialog")) {
        blocks.push({ block_type: "dialog", text: textContent });
      } else {
        // Если класс не распознан, сохраняем как дефолтное описание действия
        blocks.push({ block_type: "action_description", text: textContent });
      }
    }

    return blocks;
  }

  function executePageDeletion() {
    const targetIndex = currentIndex;
    if (targetIndex === pages.length - 1) {
      currentIndex = targetIndex - 1;
    }
    pages.splice(targetIndex, 1);
    
    // Пересчитываем ID и номера оставшихся листов сценария
    pages = pages.map((page, idx) => {
      page.id = idx + 1;
      page.number = idx + 1;
      return page;
    });

    queueMicrotask(() => {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor && pages[currentIndex]) {
        editor.innerHTML = pages[currentIndex].text || "<div><br></div>";
        focusEditor();
      }
    });
  }

  return {
    // ---- ГЕТТЕРЫ И СЕТТЕРЫ РУН SVELTE 5 ----
    get pages() { return pages; },
    set pages(v) { pages = v; },
    get currentIndex() { return currentIndex; },
    set currentIndex(v) { currentIndex = v; },
    get currentPage() { return pages[currentIndex]; },
    get showConfirmDelete() { return showConfirmDelete; },
    set showConfirmDelete(v) { showConfirmDelete = v; },

    // 1. ПОЛУЧЕНИЕ ИЗ RUST (Вызывается в команде открытия/чтения файла)
    setFromNetwork(loadedDoc: { pages: any[] }) {
      if (loadedDoc && loadedDoc.pages && loadedDoc.pages.length > 0) {
        pages = loadedDoc.pages.map((p, index) => {
          const blocks = p.blocks || [];
          // Компилируем XML структуры блоков обратно в HTML-строки с Tailwind
          const htmlText = compileBlocksToHtml(blocks);

          return {
            id: index + 1,
            number: p.number || (index + 1),
            formatting: p.formatting ?? { ...DEFAULT_SCRIPT_FORMATTING },
            blocks: blocks,
            text: htmlText,
          };
        });
      } else {
        pages = [{ id: 1, number: 1, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, blocks: [], text: "<div><br></div>" }];
      }

      currentIndex = 0;

      // Заливаем текст первой страницы в DOM-редактор
      queueMicrotask(() => {
        const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
        if (editor && pages[0]) {
          editor.innerHTML = pages[0].text;
          editor.focus();
        }
      });
    },

    // 2. СБОРКА PAYLOAD ДЛЯ RUST (Вызывается при автосохранении)
    getCompilePayload() {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor) {
        pages[currentIndex].text = editor.innerHTML;
      }

      const isPageEmpty = (htmlText: string): boolean => {
        if (!htmlText || htmlText === "<div><br></div>") return true;
        const cleanText = htmlText.replace(/<\/?[^>]+(>|$)/g, "").replace(/&nbsp;/g, "").trim();
        return cleanText.length === 0;
      };

      // Фильтруем пустые хвосты страниц в конце документа
      let filteredPages = pages.filter((p, index) => {
        if (index === 0) return true;
        return !isPageEmpty(p.text);
      });

      if (filteredPages.length !== pages.length) {
        pages = filteredPages.map((p, idx) => {
          p.id = idx + 1;
          p.number = idx + 1;
          return p;
        });

        if (currentIndex >= pages.length) {
          currentIndex = pages.length - 1;
        }

        queueMicrotask(() => {
          if (editor && pages[currentIndex]) {
            editor.innerHTML = pages[currentIndex].text || "<div><br></div>";
          }
        });
      }

      // Собираем чистый JSON-массив объектов страниц и распарсенных XML-кирпичиков блоков
      const cleanPages = pages.map((p) => {
        const currentHtml = p.id === pages[currentIndex].id && editor ? editor.innerHTML : p.text;
        
        return {
          number: p.number,
          formatting: $state.snapshot(p.formatting),
          blocks: parseHtmlToBlocks(currentHtml), // Магия парсинга HTML -> Массив структур Rust
        };
      });

      return {
        pages: cleanPages,
      };
    },

    // ---- НАВИГАЦИЯ ПО СТРАНИЦАМ ----
    next() {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor) pages[currentIndex].text = editor.innerHTML;

      if (currentIndex < pages.length - 1) {
        currentIndex++;
        queueMicrotask(() => {
          if (editor) editor.innerHTML = pages[currentIndex].text || "";
          focusEditor();
        });
      }
    },

    prev() {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor) pages[currentIndex].text = editor.innerHTML;

      if (currentIndex > 0) {
        currentIndex--;
        queueMicrotask(() => {
          if (editor) editor.innerHTML = pages[currentIndex].text || "";
          focusEditor();
        });
      }
    },

    addPage(initialText = "") {
      const editor = document.querySelector('[role="textbox"]') as HTMLDivElement;
      if (editor) pages[currentIndex].text = editor.innerHTML;

      const newId = pages.length > 0 ? Math.max(...pages.map((p) => p.id)) + 1 : 1;
      pages.push({
        id: newId,
        number: newId,
        formatting: { ...DEFAULT_SCRIPT_FORMATTING },
        blocks: [],
        text: initialText,
      });
      currentIndex = pages.length - 1;
      
      queueMicrotask(() => {
        if (editor) editor.innerHTML = initialText || "";
        focusEditor();
      });
    },

    deleteCurrentPage(onError: (msg: string) => void) {
      if (pages.length <= 1) {
        onError("Невозможно удалить страницу. В сценарии должен оставаться минимум один лист.");
        return;
      }
      showConfirmDelete = true;
    },

    confirmDeletion() { 
      showConfirmDelete = false; 
      executePageDeletion(); 
    },

    cancelDeletion() { 
      showConfirmDelete = false; 
      focusEditor(); 
    }
  };
}
          