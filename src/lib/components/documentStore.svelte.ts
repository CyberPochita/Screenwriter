import { EditorView } from "@codemirror/view";
import type { IPageContent, IPageFormatting, IScenarioDocument } from "$lib/types/titlePage";

// Дефолтные настройки полей для обычного текста сценария
const DEFAULT_SCRIPT_FORMATTING: IPageFormatting = {
  top_margin: 0,
  left_margin: 3.25,  // 3.25 см слева
  right_margin: 2.5,  // Обычное правое поле сценария
  contact_left_margin: 0
};

export function createDocumentStore() {
  let pages = $state<IPageContent[]>([
    { id: 1, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, text: "" }
  ]);
  let currentIndex = $state(0);
  let view = $state<EditorView | null>(null);

  function focusEditor() {
    queueMicrotask(() => {
      if (view) {
        view.focus();
        view.dispatch({ selection: { anchor: 0, head: 0 } });
      }
    });
  }

  return {
    get pages() { return pages; },
    set pages(v) { pages = v; },
    get currentIndex() { return currentIndex; },
    set currentIndex(v) { currentIndex = v; },
    get currentPage() { return pages[currentIndex]; },
    get view() { return view; },
    set view(v) { view = v; },

    // Восстановление всех страниц из XML файла .writer
    setFromNetwork(loadedDoc: { pages: any[] }) {
      if (loadedDoc.pages && loadedDoc.pages.length > 0) {
        pages = loadedDoc.pages.map((p, index) => ({
          id: index + 1,
          formatting: p.formatting ?? { top_margin: 0, left_margin: 3.25, right_margin: 2.5, contact_left_margin: 0 },
          // Читаем из ключа "$value", который присылает Rust Serde
          text: p["$value"] ?? p.text ?? ""
        }));
      } else {
        pages = [{ id: 1, formatting: { top_margin: 0, left_margin: 3.25, right_margin: 2.5, contact_left_margin: 0 }, text: "" }];
      }
      
      currentIndex = 0;

      // КРИТИЧЕСКИ ВАЖНО (Word-style): Принудительно обновляем текст в DOM, 
      // так как при загрузке файла pageId первой страницы не меняется (остается равным 1),
      // и стандартный эффект в Editor.svelte не срабатывает автоматически.
      queueMicrotask(() => {
        const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
        if (editorDiv && pages[0]) {
          editorDiv.innerHTML = pages[0].text;
          editorDiv.focus();
        }
      });
    },

    // Сборка для записи XML-документа на бэкенд
    getCompilePayload() {
      // ИСПРАВЛЕНО: Мапим поле 'text' в ключ '"$value"', который жестко ожидает Rust Serde
      const cleanPages = pages.map(p => ({
        formatting: $state.snapshot(p.formatting),
        "$value": p.text || "" // Переименовываем ключ для бэкенда quick-xml
      }));

      return {
        pages: cleanPages
      };
    },

    next() {
      if (currentIndex < pages.length - 1) { currentIndex++; focusEditor(); }
    },
    prev() {
      if (currentIndex > 0) { currentIndex--; focusEditor(); }
    },
    addPage(initialText = '') {
      const newId = pages.length > 0 ? Math.max(...pages.map(p => p.id)) + 1 : 1;
      // Новая страница создается со стандартными полями
      pages.push({ id: newId, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, text: initialText });
      currentIndex = pages.length - 1;
      focusEditor();
    }
  };
}
