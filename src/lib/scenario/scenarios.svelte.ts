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
      currentProject = newName;
      newName = "";
      await get_files();
    } catch (e) {
      console.error("Failed to enter project:", e);
      showToast("Не удалось взаимодействовать с проектом", true);
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
      await invoke("create_file", { name: newName });
      newName = "";
      await get_files();
      showToast(`Создан сценарий: ${newName}.writer`);
    } catch (e) {
      console.error("Failed to create scenarios:", e);
      showToast("Не удалось создать сценарий", true);
    }
  }

  async function loadContent(fileName: string) {
    try {
      const result = await invoke<any>("entry_file", { nameFile: fileName });
      console.log("РЕЗУЛЬТАТ ОТ БЭКЕНДА RUST:", result);

      if (result && Array.isArray(result)) {
        doc.setFromNetwork({ pages: result });

        chooseFile = fileName;
        navState.isVisible = !navState.isVisible;
      } else {
        currentProject = fileName;
        await get_files();
      }
    } catch (err) {
      console.error("Ошибка в загрузке контента: ", err);
      showToast(`Не удалось открыть: ${err}`, true);
    }
  }

  async function saveContent() {
    if (!chooseFile) {
      showToast("Невозможно сохранить: файл не выбран", true);
      return;
    }
    try {
      const payload = doc.getCompilePayload();
      await invoke<string>("write_to_file", {
        filePath: chooseFile,
        pages: payload.pages,
      });
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
      showToast(`Объект "${name_file}" успешно удален`);
    } catch (e) {
      console.error("Failed to delete file:", e);
      showToast(`Ошибка при удалении "${name_file}"`, true);
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
