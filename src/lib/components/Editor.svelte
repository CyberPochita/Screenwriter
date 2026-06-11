<!-- src/lib/components/Editor.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import "../../theme.css";

  let { 
    value = $bindable(''), 
    onAddPage = () => {} 
  } = $props();

  let containerRef = $state<HTMLDivElement | null>(null);
  let editorRef = $state<HTMLDivElement | null>(null);

  // Синхронизация текста: загружаем текст в редактор только при смене страницы
  // Это предотвращает разрыв фокуса курсора при обычной печати
  $effect(() => {
    if (editorRef && editorRef.innerHTML !== value) {
      editorRef.innerHTML = value || "";
    }
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === "Enter") {
      event.preventDefault();
      onAddPage();
    }
  }

  // Срабатывает во время печати
  function handleInput(event: Event) {
    if (!editorRef || !containerRef) return;

    // Сверяем высоту: если внутренний текст стал больше, чем физическая коробка А4
    if (editorRef.scrollHeight > containerRef.clientHeight) {
      // Отменяем ввод последнего символа встроенным методом браузера
      document.execCommand("undo", false);
      return;
    }

    // Если всё в порядке, сохраняем текст в переменную Svelte
    value = editorRef.innerHTML;
  }

  onMount(() => {
    editorRef?.focus();
  });
</script>

<div class="zoom-[0.75] select-none">
  <!-- 
    ВНЕШНИЙ ДИВ (containerRef): 
    Управляет внешним видом листа А4. Его высота (clientHeight) всегда стабильна 
    и равна 297mm, зум больше не сможет сломать её в JS-замерах!
  -->
  <div 
    bind:this={containerRef} 
    class="w-a4 h-a4 overflow-hidden bg-white/90 rounded-lg shadow-inner box-border relative"
  >
    
    <!-- 
      ВНУТРЕННИЙ РЕДАКТИРУЕМЫЙ ДИВ: 
      Он больше не связан через bind:textContent, поэтому буквы будут печататься 
      идеально свободно, плавно и без задержек.
    -->
    <div 
      bind:this={editorRef}
      contenteditable="true"
      role="textbox"
      tabindex="0"
      aria-label="Редактор сценария"
      onkeydown={handleKeyDown}
      oninput={handleInput} 
      class="editor-wrapper w-full h-full p-8 font-mono text-lg whitespace-pre-wrap break-words text-left outline-none box-border"
      style="outline: none;"
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
  }
</style>
