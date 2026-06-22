import { EditorView } from "@codemirror/view";
import type {
  IPageContent,
  IPageFormatting,
  IScenarioDocument,
} from "$lib/types/titlePage";

// Дефолтные настройки полей для обычного текста сценария
const DEFAULT_SCRIPT_FORMATTING: IPageFormatting = {
  top_margin: 0,
  left_margin: 3.25, // 3.25 см слева
  right_margin: 2.5, // Обычное правое поле сценария
  contact_left_margin: 0,
};

export function createDocumentStore() {
  let pages = $state<IPageContent[]>([
    { id: 1, formatting: { ...DEFAULT_SCRIPT_FORMATTING }, text: "" },
  ]);
  let currentIndex = $state(0);
  let view = $state<EditorView | null>(null);
  let showConfirmDelete = $state(false);

  function focusEditor() {
    queueMicrotask(() => {
      if (view) {
        view.focus();
        view.dispatch({ selection: { anchor: 0, head: 0 } });
      }
    });
  }

  function executePageDeletion() {
    const targetIndex = currentIndex;

    if (targetIndex === pages.length - 1) {
      currentIndex = targetIndex - 1;
    }

    pages.splice(targetIndex, 1);

    pages = pages.map((page, idx) => {
      page.id = idx + 1;
      return page;
    });

    queueMicrotask(() => {
      const editorDiv = document.querySelector(
        '[role="textbox"]',
      ) as HTMLDivElement;
      if (editorDiv && pages[currentIndex]) {
        editorDiv.innerHTML = pages[currentIndex].text || "<div><br></div>";
        focusEditor();
      }
    });
  }

  return {
    get pages() {
      return pages;
    },
    set pages(v) {
      pages = v;
    },
    get currentIndex() {
      return currentIndex;
    },
    set currentIndex(v) {
      currentIndex = v;
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
    get showConfirmDelete() {
      return showConfirmDelete;
    },
    set showConfirmDelete(v) {
      showConfirmDelete = v;
    },

    // Восстановление всех страниц из XML файла .writer
    setFromNetwork(loadedDoc: { pages: any[] }) {
      if (loadedDoc.pages && loadedDoc.pages.length > 0) {
        pages = loadedDoc.pages.map((p, index) => ({
          id: index + 1,
          formatting: p.formatting ?? {
            top_margin: 0,
            left_margin: 3.25,
            right_margin: 2.5,
            contact_left_margin: 0,
          },
          // Читаем из ключа "$value", который присылает Rust Serde
          text: p["$value"] ?? p.text ?? "",
        }));
      } else {
        pages = [
          {
            id: 1,
            formatting: {
              top_margin: 0,
              left_margin: 3.25,
              right_margin: 2.5,
              contact_left_margin: 0,
            },
            text: "",
          },
        ];
      }

      currentIndex = 0;

      // КРИТИЧЕСКИ ВАЖНО (Word-style): Принудительно обновляем текст в DOM,
      // так как при загрузке файла pageId первой страницы не меняется (остается равным 1),
      // и стандартный эффект в Editor.svelte не срабатывает автоматически.
      queueMicrotask(() => {
        const editorDiv = document.querySelector(
          '[role="textbox"]',
        ) as HTMLDivElement;
        if (editorDiv && pages[0]) {
          editorDiv.innerHTML = pages[0].text;
          editorDiv.focus();
        }
      });
    },

    // Сборка для записи XML-документа на бэкенд
    getCompilePayload() {
      // Функция-помощник: проверяет, является ли страница текстового процессора пустой
      const isPageEmpty = (htmlText: string): boolean => {
        if (!htmlText) return true;
        // Удаляем все HTML-теги, неразрывные пробелы &nbsp; и обычные пробелы/переносы
        const cleanText = htmlText
          .replace(/<\/?[^>]+(>|$)/g, "") // Вырезаем <div>, <br> и т.д.
          .replace(/&nbsp;/g, "") // Вырезаем сущности пробелов
          .trim(); // Удаляем пробелы по краям

        return cleanText.length === 0;
      };

      // 1. Фильтруем массив: оставляем только те страницы, которые НЕ пустые
      let filteredPages = pages.filter((p, index) => {
        // Первую страницу (индекс 0) мы НИКОГДА не удаляем, даже если она пустая,
        // чтобы в XML-документе всегда оставался хотя бы один лист.
        if (index === 0) return true;

        return !isPageEmpty(p.text);
      });

      // 2. Если массив изменился (какие-то пустые страницы были удалены)
      if (filteredPages.length !== pages.length) {
        // Пересчитываем ID для оставшихся страниц, чтобы CodeMirror/Svelte не теряли ключи
        pages = filteredPages.map((p, idx) => {
          p.id = idx + 1;
          return p;
        });

        // Корректируем currentIndex, если автор стоял на одной из удаленных пустых страниц
        if (currentIndex >= pages.length) {
          currentIndex = pages.length - 1;
        }

        // Принудительно обновляем текст в DOM-редакторе на текущую валидную страницу
        queueMicrotask(() => {
          const editorDiv = document.querySelector(
            '[role="textbox"]',
          ) as HTMLDivElement;
          if (editorDiv && pages[currentIndex]) {
            editorDiv.innerHTML = pages[currentIndex].text || "<div><br></div>";
          }
        });
      }

      // 3. Формируем идеально чистый payload для отправки в IPC Tauri команду Rust
      const cleanPages = pages.map((p) => ({
        formatting: $state.snapshot(p.formatting),
        $value: p.text || "",
      }));

      return {
        pages: cleanPages,
      };
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
    addPage(initialText = "") {
      const newId =
        pages.length > 0 ? Math.max(...pages.map((p) => p.id)) + 1 : 1;
      // Новая страница создается со стандартными полями
      pages.push({
        id: newId,
        formatting: { ...DEFAULT_SCRIPT_FORMATTING },
        text: initialText,
      });
      currentIndex = pages.length - 1;
      focusEditor();
    },
     deleteCurrentPage(onError: (msg: string) => void) {
      if (pages.length <= 1) {
        // Вместо alert() вызываем функцию красивого уведомления
        onError("Невозможно удалить страницу. В сценарии должен оставаться минимум один лист.");
        return;
      }
      // Если страниц больше одной — открываем наше кастомное модальное окно подтверждения
      showConfirmDelete = true;
    },

    // Метод, который будет вызываться при нажатии кнопки "Удалить" в диалоге
    confirmDeletion() {
      showConfirmDelete = false;
      executePageDeletion();
    },

    // Метод, который будет вызываться при отмене
    cancelDeletion() {
      showConfirmDelete = false;
      focusEditor();
    }
  };
}
