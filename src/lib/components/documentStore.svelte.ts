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
    deleteCurrentPage() {
      if (pages.length <= 1) {
        alert(
          "Невозможно удалить страницу. В сценарии должен оставаться минимум один лист.",
        );
        return;
      }

      const confirmDelete = confirm(
        `Вы действительно хотите удалить страницу ${currentIndex + 1}? Весь текст на ней будет безвозвратно стерт.`,
      );
      if (!confirmDelete) return;

      const targetIndex = currentIndex;

      // 1. Если удаляем самую последнюю страницу в документе — безопасно сдвигаем индекс назад
      if (targetIndex === pages.length - 1) {
        currentIndex = targetIndex - 1;
      }

      // 2. Вырезаем именно текущую страницу из реактивного массива Svelte 5
      pages.splice(targetIndex, 1);

      // 3. КРИТИЧЕСКИ ВАЖНО (Word-style): Пересчитываем ID всех оставшихся страниц.
      // Это заставит Свелт изменить pageId, что мгновенно разбудит изолированный $effect в Editor.svelte
      pages = pages.map((page, idx) => {
        page.id = idx + 1;
        return page;
      });

      // 4. Принудительно синхронизируем DOM-редактор с текстом новой текущей страницы
      queueMicrotask(() => {
        const editorDiv = document.querySelector(
          '[role="textbox"]',
        ) as HTMLDivElement;
        if (editorDiv && pages[currentIndex]) {
          // Записываем текст той страницы, которая встала на место удаленной
          editorDiv.innerHTML = pages[currentIndex].text || "<div><br></div>";
          focusEditor();
        }
      });
    },
  };
}
