<script lang="ts">
  import "./layout.css";
  import { setContext, getContext } from "svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";
  import Toast from "$lib/components/Toast.svelte";

  import { createAssetPanelManager } from "$lib/scenario/assets.svelte";
  const assets = createAssetPanelManager();

  let textEditorState = $state({
    showSettings: false,
    applyStyle: (syntax: string) => {},
  });

  let navState = $state({
    isVisible: true,
  });

  // Локальное состояние тоста для окна настроек на уровне лейаута
  let layoutToastMessage = $state<string | null>(null);
  let layoutToastIsError = $state(false);
  let layoutToastTimeout: number;

  function showLayoutToast(msg: string, isError = false) {
    clearTimeout(layoutToastTimeout);
    layoutToastMessage = msg;
    layoutToastIsError = isError;
    layoutToastTimeout = setTimeout(() => {
      layoutToastMessage = null;
    }, 3000);
  }

  setContext("nav", navState);
  setContext("editor-settings", textEditorState);

  let { children } = $props();
</script>

<div class="flex h-screen w-full overflow-hidden p-2">
  <!-- ЛЕВОЕ СИСТЕМНОЕ МЕНЮ НАВИГАЦИИ -->
  <nav
    class="flex flex-col items-center h-full bg-white/75 backdrop-blur-xl border border-white/10 rounded-2xl py-8 shadow-2xl transition-all duration-300 overflow-hidden
    {navState.isVisible
      ? 'w-30 opacity-100 mr-2'
      : 'w-0 opacity-0 p-0 border-0 m-0'}"
  >
    <!-- Пункты меню -->
    <div class="flex w-full flex-col space-y-4 flex-1">
      <a href="/scenario" class="nav-link">
        <span class="text-lg">Сценарий</span>
      </a>

      <a href="/characters" class="nav-link">
        <span class="text-lg">Персонажи</span>
      </a>

      <a href="/locations" class="nav-link">
        <span class="text-lg">Локации</span>
      </a>
    </div>

    <button
      onclick={() => (textEditorState.showSettings = true)}
      class="nav-link w-full text-center cursor-pointer outline-none hover:opacity-100 transition-opacity"
    >
      <span class="text-base">Настройки</span>
    </button>

    <a
      href="/"
      class="nav-link opacity-40 hover:opacity-100 transition-opacity mt-2"
    >
      <span class="icon text-base">Выход</span>
    </a>
  </nav>

  <!-- ОСНОВНОЙ КОНТЕНТ СТРАНИЦ -->
  <main
    class="flex-1 relative bg-white/75 backdrop-blur-md border p-2 border-white/10 rounded-2xl overflow-auto shadow-2xl"
  >
    {@render children?.()}
  </main>

  <SettingsModal
    bind:show={textEditorState.showSettings}
    onNotify={showLayoutToast}
  />

  <Toast message={layoutToastMessage} isError={layoutToastIsError} />
</div>
