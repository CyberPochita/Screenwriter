import { invoke } from "@tauri-apps/api/core";
import type { FileInfo } from "$lib/types/fileInfo";

interface NavState {
  isVisible: boolean;
}

export function createScenarioManager(navState: NavState, doc: any) {
  let files = $state<FileInfo[]>([]);
  let newName = $state("");
  let chooseFile = $state<string | null>(null);
  let currentProject = $state("scenarios");

  let toastMessage = $state<string | null>(null);
  let isToastError = $state(false);

  async function enterProject() {
    if (!newName) return;
    try {
      await invoke("enter_project", { projectName: newName });
      newName = "";
      await get_files();
    } catch (e) {
      console.error("Failed to enter project:", e);
    }
  }

  async function exitProject() {
    try {
      let result = await invoke<string | null>("exit_project");
      currentProject = result || "scenarios";
      await get_files();
    } catch (e) {
      console.error("Failed to exit project:", e);
    }
  }

  async function get_files() {
    try {
      files = await invoke("get_files");
    } catch (e) {
      console.error("Failed to load scenarios:", e);
    }
  }

  async function createScenario() {
    if (!newName) return;
    try {
      // Гарантируем, что файл создается с расширением .writer
      const fileName = newName.endsWith(".writer")
        ? newName
        : `${newName}.writer`;
      await invoke("create_file", { name: fileName });
      newName = "";
      await get_files();
    } catch (e) {
      console.error("Failed to create scenarios:", e);
    }
  }

  async function loadContent(fileName: string) {
    try {
      // Вызываем бэкенд команду. Помните, что из-за макроса ключ аргумента — nameFile
      const result = await invoke<any>("entry_file", { nameFile: fileName });

      if (result) {
        // --- ЛОГИКА: МЫ ОТКРЫЛИ ВАЛИДНЫЙ ФАЙЛ .writer ---
        // Стор сам разложит данные по полочкам и восстановит массив страниц с id
        doc.setFromNetwork(result);

        // Активируем экран редактора
        chooseFile = fileName;
        navState.isVisible = !navState.isVisible;
      } else {
        // --- ЛОГИКА: МЫ ЗАШЛИ В ДИРЕКТОРИЮ (ПАПКУ ПРОЕКТА) ---
        // 1. Обновляем реактивное имя текущего проекта в шапке архива
        currentProject = fileName;

        // 2. КРИТИЧЕСКИ ВАЖНО: Принудительно заставляем фронтенд
        // перечитать список файлов из новой папки current_dir бэкенда!
        await get_files();
      }
    } catch (err) {
      console.error("Ошибка в загрузке контента: ", err);
    }
  }

  // ИСПРАВЛЕНО: Атомарное сохранение структурированного Word-style XML документа
  async function saveContent() {
    if (!chooseFile) {
      showToast("Невозможно сохранить: файл не выбран", true);
      return;
    }

    try {
      const payload = doc.getCompilePayload();
      await invoke<string>("write_to_file", {
        document: payload,
        file: chooseFile,
      });

      // Вызываем наше красивое уведомление вместо старого alert()
      showToast("Сценарий успешно сохранен на диск", false);
    } catch (e) {
      console.error("Критическая ошибка сохранения файла сценария:", e);
      showToast(`Ошибка сохранения: ${e}`, true);
    }
  }

  function showToast(message: string, error = false) {
    toastMessage = message;
    isToastError = error;

    // Автоматически скрываем уведомление через 3 секунды
    setTimeout(() => {
      toastMessage = null;
    }, 3000);
  }

  async function deleteFile(name_file: string) {
    try {
      await invoke("delete_file", { name: name_file });
      if (chooseFile === name_file) {
        closeFile();
      } else {
        await get_files();
      }
    } catch (e) {
      console.error("Failed to delete file:", e);
    }
  }

  function closeFile() {
    chooseFile = null;
    // При закрытии сбрасываем страницы в дефолтный пустой моноширинный лист
    doc.pages = [
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
    doc.currentIndex = 0;
    navState.isVisible = !navState.isVisible;
    get_files();
  }

  function returnDir() {
    invoke("return_dir")
      .then(() => {
        currentProject = "scenarios";
      })
      .catch((e) => {
        console.error("Failed to return to directory:", e);
      });
  }

  return {
    get files() {
      return files;
    },
    get newName() {
      return newName;
    },
    set newName(value) {
      newName = value;
    },
    get chooseFile() {
      return chooseFile;
    },
    set chooseFile(value) {
      chooseFile = value;
    },
    get currentProject() {
      return currentProject;
    },
    get toastMessage() {
      return toastMessage;
    },
    set toastMessage(value) {
      toastMessage = value;
    },
    get isToastError() {
      return isToastError;
    },
    set isToastError(value) {
      isToastError = value;
    },
    showToast,
    enterProject,
    exitProject,
    get_files,
    createScenario,
    loadContent,
    saveContent,
    deleteFile,
    closeFile,
    returnDir,
  };
}
