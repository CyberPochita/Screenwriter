<script lang="ts">
  import type { EditorView } from "@codemirror/view";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { getContext } from "svelte";

  import "../../../lib/components/EditPanel.svelte";
  import Editor from "../../../lib/components/Editor.svelte";

  interface NavState {
    isVisible: boolean;
  }

  let navState = getContext("nav") as NavState;
  let editorSettings = getContext<any>("editor-settings");
  let files = $state<string[]>([]);
  let newName = $state("");
  let chooseFile = $state<string | null>(null);
  let content = $state("");
  let view = $state<EditorView | null>(null);
  let currentProject = $state("scenarios");

  editorSettings.applyStyle = (syntax: string) => {
    if (!view) return;
    const { state, dispatch } = view;
    const { from, to } = state.selection.main;
    const selectedText = state.sliceDoc(from, to);

    let insertion: string;
    let newCursorFrom: number;
    let newCursorTo: number;

    const isWrapped =
      state.sliceDoc(from - syntax.length, from) === syntax &&
      state.sliceDoc(to, to + syntax.length) === syntax;

    if (isWrapped) {
      dispatch({
        changes: {
          from: from - syntax.length,
          to: to + syntax.length,
          insert: selectedText,
        },
        selection: {
          anchor: from - syntax.length,
          head: to - syntax.length,
        },
      });
    } else {
      if (syntax.endsWith(" ")) {
        insertion = `${syntax}${selectedText}`;
        newCursorFrom = from + syntax.length;
        newCursorTo = to + syntax.length;
      } else {
        insertion = `${syntax}${selectedText}${syntax}`;
        newCursorFrom = from + syntax.length;
        newCursorTo = to + syntax.length;
      }
      dispatch({
        changes: { from, to, insert: insertion },
        selection: { anchor: from + syntax.length, head: to + syntax.length },
      });
    }
    view.focus();
  };

  $effect(() => {
    editorSettings.showSettings = !!chooseFile;
  });

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
        content = result;
        chooseFile = name_file;
        navState.isVisible = !navState.isVisible;
      }
    } catch (error) {
      console.error("Ошибка в загрузке контента: ", error);
    }
  }

  async function saveContent() {
    await invoke("write_to_file", { msg: content, file: chooseFile });
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
    content = "";
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

  onMount(() => {
    get_files();
  });

  onDestroy(() => {
    editorSettings.showSettings = false;
    returnDir();
  });
</script>

<div class="h-full p-5 font-serif">
  {#if chooseFile}
    <!-- ЭКРАН РЕДАКТОРА -->
    <div class="flex flex-col h-full animate-in fade-in duration-500">
      <header
        class="mb-6 flex justify-between items-center border-b border-black/5 pb-4"
      >
        <button
          onclick={closeFile}
          class="font-mono text-xs opacity-40 hover:opacity-100 transition-opacity"
        >
          ← ВЕРНУТЬСЯ В АРХИВ
        </button>
        <div class="text-center">
          <h2 class="text-xl italic">{chooseFile}</h2>
          <span
            class="font-mono text-[9px] uppercase tracking-widest opacity-30"
            >Режим редактирования</span
          >
        </div>
        <button
          onclick={saveContent}
          class="bg-black text-white px-4 py-1 text-xs font-mono uppercase rounded-sm hover:bg-gray-800 transition-colors"
        >
          Сохранить
        </button>
      </header>

      <!-- Само содержимое файла -->
      <div class="editor-wrapper flex-1 overflow-auto font-mono text-lg">
        <Editor bind:value={content} bind:view />
      </div>
    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (АРХИВ) -->
    <header class="mb-10 flex justify-between items-end">
      <div>
        <h1 class="text-3xl italic">Архив рукописей</h1>
        {#if currentProject !== "scenarios"}
          <button
            onclick={exitProject}
            class="font-mono text-xs opacity-40 hover:opacity-100 transition-opacity"
          >
            ← НАЗАД
          </button>
        {/if}
      </div>

      <div class="">
        <p class="font-mono text-xl">
          {#if currentProject}
            Папка: <span class="italic">{currentProject}</span>
          {:else}
            Выберите проект или создайте новый
          {/if}
        </p>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={newName}
          placeholder="Имя файла/проекта..."
          class="border-b border-black/10 bg-transparent px-2 text-xs outline-none"
        />

        <button
          onclick={enterProject}
          class="border border-black/10 px-4 py-1 text-xs font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать проект
        </button>
        <button
          onclick={createScenario}
          class="border border-black/10 px-4 py-1 text-xs font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать сценарий
        </button>
      </div>
    </header>

    <div class="flex flex-col gap-3">
      {#each files as file}
        <!-- Контейнер для пары кнопок -->
        <div
          class="group flex items-stretch gap-0 hover:gap-3 w-full transition-all duration-300"
        >
          <!-- Основная кнопка -->
          <button
            onclick={() => loadContent(file)}
            class="flex-1 flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all text-left"
          >
            <span class="text-xl text-writer-dark/80">{file}</span>
            <span
              class="font-mono text-[10px] opacity-0 group-hover:opacity-40 tracking-tighter transition-opacity duration-300"
            >
              ЧИТАТЬ →
            </span>
          </button>

          <!-- Кнопка удаления с плавной анимацией появления -->
          <button
            onclick={() => deleteFile(file)}
            class="flex items-center justify-center bg-white/40 border border-white/10 rounded-2xl hover:bg-red-500 hover:text-white text-left
               w-0 opacity-0 pointer-events-none overflow-hidden hover:shadow-xl hover:-translate-y-0.5
               group-hover:w-14 group-hover:opacity-100 group-hover:pointer-events-auto
               transition-all duration-300 ease-out"
          >
            <span class="font-mono text-[12px] tracking-tighter"> X </span>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>
