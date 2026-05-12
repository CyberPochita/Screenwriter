<script lang="ts">
  import "./layout.css";
  import { setContext } from "svelte";
  // import TextSettings from "../../lib/components/EditPanel.svelte";
  import EditPanel from "../../lib/components/EditPanel.svelte";

  let textEditorState = $state({
    showSettings: false,
    applyStyle: (syntax: string) => {} 
  });
  
  let navState = $state({
    isVisible: true,
  });
  
  setContext("nav", navState);
  setContext('editor-settings', textEditorState);

  let { children } = $props();
</script>

<div class="flex h-screen w-full overflow-hidden p-2">
  <!-- {#if navState.isVisible}
    <button
      class="absolute left-4 top-1/2 -translate-y-1/2 z-50 p-2 bg-white/75 backdrop-blur-md border border-white/10 rounded-full shadow-lg hover:bg-white transition-all"
    >
      <span class="text-xs font-bold">>></span>
    </button>
    {/if} -->

  <!-- Островок навигации -->
  <nav
    class="flex flex-col items-center h-full bg-white/75 backdrop-blur-xl border border-white/10 rounded-2xl py-8 shadow-2xl transition-all duration-300 overflow-hidden
    {navState.isVisible
      ? 'w-30 opacity-100 mr-2'
      : 'w-0 opacity-0 p-0 border-0 m-0'}"
  >
    <!-- Пункты меню -->
    <div class="flex w-full flex-col space-y-4 flex-1">
      <a href="/scenario" class="nav-link">
        <span class="text-sm">Сценарий</span>
      </a>

      <a href="/characters" class="nav-link">
        <span class="text-sm">Персонажи</span>
      </a>

      <a href="/locations" class="nav-link">
        <span class="text-sm">Локации</span>
      </a>

      <a href="/board" class="nav-link">
        <span class="text-sm">Доска</span>
      </a>
    </div>

    <!-- Нижняя кнопка Настройки/Выход -->
    <a href="/settings" class="nav-link">
      <span class="text-sm">Настройки</span>
    </a>
    <a
      href="/"
      class="nav-link opacity-30 hover:opacity-100 transition-opacity mt-2"
    >
      <span class="icon text-sm">Выход</span>
    </a>
  </nav>

  <!-- Основной контент -->
  <main
    class="flex-1 relative bg-white/75 backdrop-blur-md border p-2 border-white/10 rounded-2xl overflow-auto shadow-2xl"
  >
    {@render children?.()}
  </main>

  {#if textEditorState.showSettings}
    <EditPanel onApplyStyle={(s: any) => textEditorState.applyStyle(s)} />
  {/if}
</div>
