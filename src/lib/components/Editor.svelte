<!-- src/lib/components/Editor.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import "../../theme.css";

  let { 
    value = $bindable(''), 
    pageId = 1,
    onAddPage = () => {} 
  } = $props();

  let containerRef = $state<HTMLDivElement | null>(null);
  let editorRef = $state<HTMLDivElement | null>(null);

  // СИНХРОНИЗАЦИЯ СТРАНИЦ: Срабатывает СТРОГО при перелистывании (смене pageId)
  $effect(() => {
    // Подписываемся на смену ID страницы (перелистывание) и на явное изменение текста извне (загрузка)
    const _pageTrigger = pageId; 
    const _textTrigger = value;

    if (editorRef && editorRef.innerHTML !== value) {
      // Загружаем HTML-разметку (сохраняя &nbsp; и теги строк из XML)
      editorRef.innerHTML = value || "<div><br></div>";
    }
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === "Enter") {
      event.preventDefault();
      onAddPage();
    }
  }

  // Срабатывает при печати на текущей странице
  function handleInput(event: Event) {
    if (!editorRef || !containerRef) return;

    // Валидация переполнения листа А4
    if (editorRef.scrollHeight > containerRef.clientHeight) {
      document.execCommand("undo", false);
      return;
    }

    // Сохраняем в XML-структуру HTML-код (с &nbsp;), чтобы верстка не терялась
    value = editorRef.innerHTML;
  }

  onMount(() => {
    editorRef?.focus();
  });
</script>

<div class="zoom-[0.75] select-none">
  <div bind:this={containerRef} class="w-a4 h-a4 overflow-hidden bg-white rounded-lg shadow-inner box-border relative">
    <div 
      bind:this={editorRef}
      contenteditable="true"
      role="textbox"
      tabindex="0"
      aria-label="Редактор сценария"
      onkeydown={handleKeyDown}
      oninput={handleInput} 
      class="editor-wrapper w-full h-full p-4 font-mono text-lg text-left outline-none box-border"
      style="outline: none; white-space: pre-wrap; word-break: break-all;"
    >
    </div>
  </div>
</div>

<style>
  .editor-wrapper {
    display: block;
    cursor: text;
    line-height: 1.6;
    color: #1e1e1e;
    font-family: 'Courier Prime', 'Courier New', Courier, monospace;
    font-size: 16px;
  }
</style>
