<script lang="ts">
  import CodeMirror from "svelte-codemirror-editor";
  import { markdown } from "@codemirror/lang-markdown";
  import type { EditorView } from "@codemirror/view";

  // Используем $bindable для связи с родителем
  let { 
    value = $bindable(''), 
    view = $bindable(null) 
  } = $props();
</script>

<div class="editor-wrapper h-full flex-1 overflow-auto font-mono text-lg">
  <CodeMirror
    bind:value={value}
    onready={(v) => (view = v)}
    lang={markdown()}
    rectangularSelection={false}
    lineWrapping={true}
    styles={{
      "&": { height: "100%", backgroundColor: "transparent" },
      "&.cm-focused": { outline: "none" },
      ".cm-scroller": { fontFamily: "var(--font-mono)", lineHeight: "1.8" },
      ".cm-content": { padding: "10px" },
      ".cm-gutters": { 
        backgroundColor: "transparent", 
        borderRight: "1px solid rgba(25, 25, 25, 0.2)", 
        color: "rgba(25, 25, 25, 0.5)", 
        borderLeft: "none" 
      },
      ".cm-activeLineGutter": { 
        backgroundColor: "transparent", 
        color: "var(--color-writer-focus)" 
      },
    }}
  />
</div>

<style>
  :global(.cm-editor) {
    height: 100%;
    outline: none !important;
  }
</style>
