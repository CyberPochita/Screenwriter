<!-- src/lib/components/editor.svelte -->
<script lang="ts">
  import CodeMirror from "svelte-codemirror-editor";
  import "../../theme.css";
  
  let { 
    value = $bindable(''), 
    view = $bindable(null) 
  } = $props();
</script>

<div class="zoom-[0.75] select-none">
  <div class="editor-wrapper w-a4 h-a4 overflow-hidden font-mono text-lg bg-white/90 rounded-lg p-4 shadow-inner flex flex-col">
    <CodeMirror
      bind:value={value}
      onready={(v) => (view = v)}
      rectangularSelection={false}
      lineWrapping={true}
      lineNumbers={false}
      foldGutter={false}
      highlight={false as any}
    />
  </div>
</div>

<style>
  :global(.cm-editor) {
    outline: none !important;
  }
  
  :global(.cm-scroller) {
    overflow: hidden !important; 
  }
</style>
