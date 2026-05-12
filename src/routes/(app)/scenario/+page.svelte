<script lang="ts">
  import CodeMirror from "svelte-codemirror-editor";
  import { markdown } from "@codemirror/lang-markdown";
  import type { EditorView } from "@codemirror/view";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { getContext } from "svelte";

  import "../../../lib/components/EditPanel.svelte";
  import EditPanel from "../../../lib/components/EditPanel.svelte";

  interface NavState {
    isVisible: boolean;
  }

  let navState = getContext("nav") as NavState;
  let scenarios = $state<string[]>([]);
  let newName = $state("");
  //let path = $state('');
  let chooseFile = $state<string | null>(null);

  let editorSettings = getContext<any>("editor-settings");

  editorSettings.applyStyle = (syntax: string) => {
    if (!view) return;
    const { state, dispatch } = view;
    const { from, to } = state.selection.main;
    const selectedText = state.sliceDoc(from, to);

    dispatch({
      changes: { from, to, insert: `${syntax}${selectedText}${syntax}` },
      selection: { anchor: from + syntax.length, head: to + syntax.length },
    });
    view.focus();
  };

  $effect(() => {
    editorSettings.showSettings = !!chooseFile;
  });

  let content = $state("");
  let view = $state<EditorView | null>(null);

  // const handleEditorInit = (v: EditorView) => {
  //   view = v;
  // };

  function applyMarkdown(syntax: string) {
    if (!view) return;

    const { state, dispatch } = view;
    const { from, to } = state.selection.main;
    const selectedText = state.sliceDoc(from, to);

    let insertion: string;
    let newCursorPos: number;

    if (syntax.endsWith(" ")) {
      insertion = `${syntax}${selectedText}`;
      newCursorPos = to + syntax.length;
    } else {
      insertion = `${syntax}${selectedText}${syntax}`;
      newCursorPos = to + syntax.length;
    }

    dispatch({
      changes: { from, to, insert: `${syntax}${selectedText}${syntax}` },
      selection: { anchor: from + syntax.length, head: to + syntax.length },
    });

    view.focus();
  }

  // let textSettings = getContext<any>('textSettings');

  async function loadScenarios() {
    try {
      scenarios = await invoke("get_scenarios");
    } catch (e) {
      console.error("Failed to load scenarios:", e);
    }
  }

  async function createScenario() {
    if (!newName) return;
    try {
      await invoke("create_file", { name: newName });
      newName = "";
      await loadScenarios();
    } catch (e) {
      console.error("Failed to create scenario:", e);
    }
  }

  async function loadContent(name_file: string) {
    try {
      content = await invoke("read_file", { nameFile: name_file });
      chooseFile = name_file;
      navState.isVisible = !navState.isVisible;
      // textSettings.show = true;
    } catch (e) {
      console.error("Failed to load content:", e);
      console.error("Name file:", name_file);
    }
  }

  async function saveContent() {
    await invoke("write_to_file", { msg: content, file: chooseFile });
  }

  function closeFile() {
    chooseFile = null;
    content = "";
    navState.isVisible = !navState.isVisible;
    // EditPanel.show = false;
    loadScenarios();
  }

  onMount(() => {
    loadScenarios();
  });

  onDestroy(() => {
    editorSettings.showSettings = false;
  });
</script>

<!-- <EditPanel onApplyStyle={applyMarkdown} /> -->

<div class="h-full p-8 font-serif">
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
      <div class="h-full w-full bg-transparent overflow-hidden flex flex-col">
        <div class="editor-wrapper flex-1 overflow-auto font-mono text-lg">
          <CodeMirror
            value={content}
            onchange={(v) => (content = v)}
            onready={(v) => (view = v)}
            lang={markdown()}
            styles={{
              "&": { height: "100%", backgroundColor: "transparent" },
              ".cm-scroller": {
                fontFamily: "var(--font-mono)",
                lineHeight: "1.8",
              },
              ".cm-content": { padding: "40px" },
              ".cm-gutters": {
                backgroundColor: "transparent",
                border: "none",
                opacity: "0.2",
              },
            }}
            lineWrapping={true}
          />
        </div>
      </div>
    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (АРХИВ) -->
    <header class="mb-10 flex justify-between items-end">
      <div>
        <h1 class="text-3xl italic">Архив рукописей</h1>
        <p
          class="font-mono text-[10px] opacity-40 mt-2 uppercase tracking-widest"
        >
          Scriptwriter_OS // Storage
        </p>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={newName}
          placeholder="Имя файла..."
          class="border-b border-black/10 bg-transparent px-2 text-xs outline-none"
        />
        <button
          onclick={createScenario}
          class="border border-black/10 px-4 py-1 text-xs font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать
        </button>
      </div>
    </header>

    <div class="grid gap-3">
      {#each scenarios as file}
        <button
          onclick={() => loadContent(file)}
          class="flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all group text-left"
        >
          <span class="text-xl text-writer-dark/80">{file}</span>
          <span
            class="font-mono text-[10px] opacity-0 group-hover:opacity-40 tracking-tighter"
            >ЧИТАТЬ →</span
          >
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  /* Скрываем стандартные рамки CodeMirror, чтобы вписать в АРМ */
  :global(.cm-editor) {
    outline: none !important;
  }
</style>
