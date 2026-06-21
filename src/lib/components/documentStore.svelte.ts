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
    setFromNetwork(loadedDoc: IScenarioDocument) {
      if (loadedDoc.pages && loadedDoc.pages.length > 0) {
        pages = loadedDoc.pages.map((p, index) => ({
          id: index + 1,
          formatting: p.formatting ?? { ...DEFAULT_SCRIPT_FORMATTING },
          text: p.text ?? ""
        }));
      } else {
        pages = [{ id: 1, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, text: "" }];
      }
      currentIndex = 0;
      focusEditor();
    },

    // Сборка для записи XML-документа на бэкенд
    getCompilePayload() {
      return {
        // Очищаем от ID перед отправкой в Rust
        pages: pages.map(p => ({
          formatting: $state.snapshot(p.formatting),
          text: p.text
        }))
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
