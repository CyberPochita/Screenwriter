import { EditorView } from "@codemirror/view";

export function createDocumentStore() {
  let pages = $state([{ id: 1, text: "" }]);
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
    get pages() {
      return pages;
    },
    get currentIndex() {
      return currentIndex;
    },
    get currentPage() {
      return pages[currentIndex];
    },
    get view() {
      return view;
    },

    set view(v) {
      view = v;
    },

    next() {
      if (currentIndex < pages.length - 1) {
        currentIndex++;
        focusEditor();
      }
    },
    prev() {
      if (currentIndex > 0) {
        currentIndex--;
        focusEditor();
      }
    },
    addPage(initialText = '') {
      const newId = pages.length > 0 ? Math.max(...pages.map(p => p.id)) + 1 : 1;
      pages.push({ id: newId, text: initialText });
      currentIndex = pages.length - 1;

      focusEditor();
    },
  };
}
