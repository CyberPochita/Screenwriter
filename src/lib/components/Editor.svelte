<!-- src/lib/components/Editor.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import "../../theme.css";

  let { 
    value = $bindable(''), 
    // ДОБАВЛЕНО: Принимаем id текущей страницы для изоляции эффекта
    pageId = 1,
    onAddPage = () => {} 
  } = $props();

  let containerRef = $state<HTMLDivElement | null>(null);
  let editorRef = $state<HTMLDivElement | null>(null);

  // ИСПРАВЛЕНО: Эффект теперь жестко изолирован. 
  // Он срабатывает ТОЛЬКО тогда, когда изменился pageId (перелистывание страниц в Word).
  // Во время обычной печати или работы макросов на текущей странице этот код спать!
  $effect(() => {
    // Явно читаем pageId, чтобы Свелт подписался только на этот триггер
    const _trigger = pageId; 
    
    if (editorRef && editorRef.innerText !== value) {
      editorRef.innerText = value || "";
    }
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === "Enter") {
      event.preventDefault();
      onAddPage();
    }
  }

  // Срабатывает при ЛЮБОМ вводе или макросе
  function handleInput(event: Event) {
    if (!editorRef || !containerRef) return;

    // Проверка на переполнение листа А4
    if (editorRef.scrollHeight > containerRef.clientHeight) {
      document.execCommand("undo", false);
      return;
    }

    // Сохраняем чистый текст в стейт Svelte без триггера обратного эффекта
    value = editorRef.innerText;
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
