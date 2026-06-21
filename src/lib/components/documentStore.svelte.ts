import { EditorView } from "@codemirror/view";
import type { ITitlePage } from "$lib/types/titlePage";

export function createDocumentStore() {
  // Существующее состояние страниц редактора CodeMirror
  let pages = $state([{ id: 1, text: "" }]);
  let currentIndex = $state(0);
  let view = $state<EditorView | null>(null);
  let activeTab = $state<'editor' | 'title'>('editor');

  // НОВОЕ: Полностью реактивное состояние титульного листа сценария
  let titlePage = $state<ITitlePage>({
    title: '',
    author: '',
    authorship: 'original',
    contact: { 
      name: '',
      address: null, // Теперь TS без проблем разрешает присваивать null
      phone: null,
      email: null,
      agent: null
    }
  });

  function focusEditor() {
    queueMicrotask(() => {
      if (view) {
        view.focus();
        view.dispatch({ selection: { anchor: 0, head: 0 } });
      }
    });
  }

  return {
    // Геттеры и сеттеры страниц и вьювера
    get pages() { return pages; },
    set pages(v) { pages = v; },
    get currentIndex() { return currentIndex; },
    set currentIndex(v) { currentIndex = v; },
    get currentPage() { return pages[currentIndex]; },
    get view() { return view; },
    set view(v) { view = v; },
    get activeTab() { return activeTab; },
    set activeTab(v) { activeTab = v; },

    // НОВОЕ: Геттер для получения реактивной структуры титульного листа в UI
    get titlePage() { return titlePage; },

    // НОВОЕ: Метод восстановления стора из XML-документа, пришедшего от Rust
    setFromNetwork(loadedDoc: { title_page: ITitlePage; pages: any[] }) {
      // Инициализируем титульный лист
      titlePage = loadedDoc.title_page;
      
      // Преобразуем массив объектов от Rust с учетом переименования Serde ($value)
      if (loadedDoc.pages && loadedDoc.pages.length > 0) {
        pages = loadedDoc.pages.map((p, index) => ({
          id: index + 1,
          // Читаем из ключа "$value", который присылает бэкенд. 
          // Если страница абсолютно пустая, подставляем ""
          text: p["$value"] ?? p.text ?? ""
        }));
      } else {
        pages = [{ id: 1, text: "" }];
      }
      
      currentIndex = 0;
      focusEditor();
    },

    // НОВОЕ: Сборка и очистка данных (Snapshot) для безопасной отправки в IPC Tauri команду write_to_file
    getCompilePayload() {
      // Гарантируем, что даже если текст пустой, мы передаем ""
      const cleanPages = pages.map(p => ({ text: p.text || "" }));
      const snapshotTitle = $state.snapshot(titlePage);

      // Ваша существующая очистка контактов...
      if (snapshotTitle.contact) {
        if (snapshotTitle.contact.address === "") snapshotTitle.contact.address = null;
        if (snapshotTitle.contact.phone === "") snapshotTitle.contact.phone = null;
        if (snapshotTitle.contact.email === "") snapshotTitle.contact.email = null;
      }

      return {
        title_page: snapshotTitle,
        pages: cleanPages
      };
    },

    // Ваши оригинальные методы навигации и создания страниц
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
