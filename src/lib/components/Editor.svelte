<!-- src/lib/components/Editor.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import "../../theme.css";

  let { value = $bindable(""), pageId = 1, onAddPage = () => {} } = $props();

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
      return;
    }

    // ИСПРАВЛЕНО: Если нажат обычный Enter, принудительно разрываем наследование стилей реплики
    if (event.key === "Enter" && !event.ctrlKey) {
      event.preventDefault();

      // Нативно вставляем абсолютно чистый пустой блок строки,
      // тем самым сбрасывая любые ограничения маргинов и пробелов для следующей строки!
      document.execCommand("insertHTML", false, "<div><br></div>");

      // Обновляем стейт Svelte
      if (editorRef) value = editorRef.innerHTML;
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

<div class="zoom-[0.75]">
  <div
    bind:this={containerRef}
    class="w-a4 h-a4 overflow-hidden bg-white rounded-lg shadow-inner box-border relative"
  >
    <div
      bind:this={editorRef}
      contenteditable="true"
      role="textbox"
      tabindex="0"
      aria-label="Редактор сценария"
      onkeydown={handleKeyDown}
      oninput={handleInput}
      class="editor-wrapper w-full h-full p-4 font-mono text-lg text-left outline-none box-border"
      style="outline: none; white-space: pre-wrap; word-break: break-word;"
    ></div>
  </div>
</div>

<style>
  .editor-wrapper {
    display: block;
    cursor: text;
    line-height: 1.6;
    color: #1e1e1e;
    font-family: "Courier Prime", "Courier New", Courier, monospace;
    font-size: 16px;
  }

  .editor-wrapper :global(.script-dialogue) {
    display: block !important;
    width: auto !important; /* Разрешаем блоку сжиматься с краев */

    /* Жесткое левое поле: 7.5cm всего - 3.25cm (базовое поле листа) = 4.25cm */
    margin-left: calc(7.5cm - 3.25cm) !important;

    /* Жесткое правое поле: 6.25cm всего - 2.5cm (базовое поле листа) = 3.75cm */
    margin-right: calc(6.25cm - 2.5cm) !important;

    /* Сбрасываем внутренние отступы, чтобы текст не слипался */
    padding: 0 !important;

    /* Гарантируем правильный перенос слов браузером */
    white-space: pre-wrap !important;
    word-break: break-word !important;
  }

  .editor-wrapper :global(.script-parenthetical) {
    display: block !important;
    width: auto !important;

    /* Вычисляем левое поле: 9.25cm всего - 3.25cm (базовое поле листа) = 6.0cm */
    margin-left: calc(9.25cm - 3.25cm) !important;

    /* Вычисляем правое поле: 6.25cm всего - 2.5cm (базовое поле листа) = 3.75cm */
    margin-right: calc(6.25cm - 2.5cm) !important;

    padding: 0 !important;
    white-space: pre-wrap !important;
    word-break: break-word !important;
  }

  .editor-wrapper :global(.script-title-text) {
    display: block !important;
    width: auto !important;

    /* Копируем левое поле ремарки: 9.25cm всего - 3.25cm базового поля = 6.0cm */
    margin-left: calc(9.25cm - 3.25cm) !important;

    /* Копируем правое поле ремарки: 6.25cm всего - 2.5cm базового поля = 3.75cm */
    margin-right: calc(6.25cm - 2.5cm) !important;

    padding: 0 !important;
    white-space: pre-wrap !important;
    word-break: break-word !important;
  }
</style>
