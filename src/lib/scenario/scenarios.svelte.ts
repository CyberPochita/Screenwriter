import { invoke } from "@tauri-apps/api/core";

interface NavState {
  isVisible: boolean;
}

export function createScenarioManager(navState: NavState, doc: any) {
  let files = $state<string[]>([]);
  let newName = $state("");
  let chooseFile = $state<string | null>(null);
  let currentProject = $state("scenarios");

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
      await invoke("create_file", { name: newName });
      newName = "";
      await get_files();
    } catch (e) {
      console.error("Failed to create scenarios:", e);
    }
  }

  async function loadContent(name_file: string) {
    try {
      const result = await invoke<string | null>("entry_file", {
        nameFile: name_file,
      });
      console.log("Результат от функции Rust: ", result);
      if (result === null) {
        currentProject = name_file;
        get_files();
      } else {
        doc.pages[0].text = result || "";
        doc.currentIndex = 0;
        chooseFile = name_file;
        navState.isVisible = !navState.isVisible;
      }
    } catch (error) {
      console.error("Ошибка в загрузке контента: ", error);
    }
  }

  async function saveContent() {
    const fullText = doc.pages.map((p: any) => p.text).join("\n");
    await invoke("write_to_file", { msg: fullText, file: chooseFile });
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
    doc.pages = [{ id: 1, text: "" }];
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
