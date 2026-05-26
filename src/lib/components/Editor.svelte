<script lang="ts">
  import CodeMirror from "svelte-codemirror-editor";
  
  // Используем $bindable для связи с родителем
  let { 
    value = $bindable(''), 
    view = $bindable(null) 
  } = $props();
</script>

<div class="editor-wrapper h-full flex-1 overflow-auto font-mono text-lg bg-white/90 rounded-lg p-4 shadow-inner">
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

<style>
  :global(.cm-editor) {
    height: 100%;
    outline: none !important;
  }
</style>
