<script lang="ts">
  import { onMount, onDestroy, getContext, setContext } from "svelte";
  import { createDocumentStore } from "$lib/components/documentStore.svelte";
  import { createScenarioManager } from "$lib/scenario/scenarios.svelte";
  import { fade } from "svelte/transition";
  import "$lib/components/EditPanel.svelte";
  import Editor from "$lib/components/Editor.svelte";
  import EditPanel from "$lib/components/EditPanel.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
  import { createAssetPanelManager } from "$lib/scenario/assets.svelte";
  import AssetPanel from "$lib/components/AssetPanel.svelte";
  import CharacterTooltip from "$lib/components/CharacterTooltip.svelte"; 

  interface NavState {
    isVisible: boolean;
  }

  let navState = getContext("nav") as NavState;
  let editorSettings = getContext<any>("editor-settings");
  const doc = createDocumentStore();
  const manager = createScenarioManager(navState, doc);
  const assets = createAssetPanelManager();

  setContext("doc", doc);

  onMount(() => {
    manager.get_files();
  });

  onDestroy(() => {
    editorSettings.showSettings = false;
    manager.returnDir();
  });
</script>

<div class="h-full p-5 font-serif">
  {#if manager.chooseFile}
    <!-- ЭКРАН РЕДАКТОРА (ЕДИНЫЙ ТЕКСТОВЫЙ ПРОЦЕССОР) -->
    <div
      class="flex flex-col h-full animate-in fade-in duration-500 text-slate-900"
    >
      <header
        class="flex justify-between items-center border-b border-black/5 bg-white pl-4 pr-4 shadow-sm rounded-2xl"
      >
        <button
          onclick={manager.closeFile}
          class="font-mono text-sm opacity-40 hover:opacity-100 transition-opacity"
        >
          ❮ ВЕРНУТЬСЯ В АРХИВ
        </button>

        <div class="text-center">
          <h2 class="text-xl italic">{manager.chooseFile}</h2>
          <span class="font-mono text-xs uppercase tracking-widest opacity-50">
            Страница {doc.currentIndex + 1}
          </span>
        </div>

        <button
          onclick={manager.saveContent}
          class="bg-black text-white px-5 py-1.5 text-sm font-mono uppercase rounded-sm hover:bg-gray-800 transition-colors"
        >
          Сохранить
        </button>
      </header>

      <!-- РАБОЧАЯ ОБЛАСТЬ СТРАНИЦЫ -->
      <div 
        role="document"
        tabindex="-1"
        class="flex-1 flex justify-center items-start overflow-auto min-h-0 outline-none"
      >
        <div class="relative my-4 rounded-sm">
          {#if doc.currentPage}
            <Editor
              bind:value={doc.currentPage.text}
              pageId={doc.currentPage.id}
              onAddPage={doc.addPage}
              onMouseMove={(e) => assets.handleMouseMove(e)}
            />
          {/if}

          <!-- БЛОК НАВИГАЦИИ СТРАНИЦ (Стрелочки справа от листа) -->
          <div
            class="absolute top-4 left-[calc(100%+24px)] flex flex-col gap-1 text-black/70 font-mono select-none w-55"
          >
            <p class="text-m uppercase tracking-wider">
              Количество страниц: {doc.pages.length}
            </p>
            <!-- TODO: Настроить счетчики -->
            <p class="text-m uppercase tracking-wider">Количество букв: 0</p>
            <p class="text-m uppercase tracking-wider">Количество слов: 0</p>
          </div>

          <div
            class="absolute bottom-0 left-[calc(100%+24px)] flex flex-col gap-3 w-40 select-none"
          >
            <div class="flex flex-col gap-3">
              <!-- Кнопка создания новой страницы -->
              <button
                onclick={() => doc.addPage()}
                class="w-full py-2.5 px-4 bg-white border border-black/10 text-black/70 rounded-xl font-sans text-sm font-semibold hover:bg-black hover:text-white hover:shadow-md transition-all cursor-pointer text-center"
              >
                Создать страницу
              </button>

              <button
                onclick={() =>
                  doc.deleteCurrentPage((msg) => manager.showToast(msg, true))}
                class="w-full py-2.5 px-4 bg-white border border-black/10 text-black/70 rounded-xl font-sans text-sm font-semibold hover:bg-black hover:text-white hover:shadow-md transition-all cursor-pointer text-center"
              >
                Удалить страницу
              </button>

              <div class="flex gap-1 h-11">
                <button
                  onclick={doc.prev}
                  disabled={doc.currentIndex === 0}
                  class="flex-1 flex items-center justify-center p-3 bg-white text-black/50 border border-black/10 rounded-2xl hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer font-bold"
                >
                  ❮
                </button>
                <button
                  onclick={doc.next}
                  disabled={doc.currentIndex === doc.pages.length - 1}
                  class="flex-1 flex items-center justify-center p-3 bg-white border border-black/10 rounded-2xl text-black/50 hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer font-bold"
                >
                  ❯
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <EditPanel />
      <AssetPanel {assets} />
      <CharacterTooltip {assets} />
    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (АРХИВ) -->
    <header class="mb-10 flex justify-between items-end">
      <div class="relative pb-5">
        <h1 class="text-3xl italic">Сценарии</h1>
        {#if manager.currentProject !== "scenarios"}
          <button
            transition:fade={{ duration: 200 }}
            onclick={manager.exitProject}
            class="absolute bottom-0 left-0 font-mono text-sm opacity-40 hover:opacity-100 transition-opacity whitespace-nowrap"
          >
            ← НАЗАД
          </button>
        {/if}
      </div>

      <div class="">
        <p class="font-mono text-xl">
          {#if manager.currentProject}
            Проект: <span class="italic">{manager.currentProject}</span>
          {:else}
            Выберите проект или создайте новый
          {/if}
        </p>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={manager.newName}
          placeholder="Имя файла/проекта..."
          class="border-b border-black/10 bg-transparent px-2 text-sm outline-none"
        />

        <button
          onclick={manager.enterProject}
          class="border border-black/10 px-4 py-1 text-sm font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать проект
        </button>
        <button
          onclick={manager.createScenario}
          class="border border-black/10 px-4 py-1 text-sm font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать сценарий
        </button>
      </div>
    </header>

    <div class="flex flex-col gap-3">
      {#each manager.files as file}
        <!-- Контейнер для пары кнопок -->
        <div
          class="group flex items-stretch gap-0 hover:gap-3 w-full transition-all duration-300"
        >
          <button
            onclick={() => manager.loadContent(file.file_name)}
            class="flex-1 flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all text-left"
          >
            <span class="text-xl text-writer-dark/80">{file.file_name}</span>
            {#if file.file_format === ""}
              <span class="text-xl text-writer-dark/80">Проект</span>
            {/if}
            <span
              class="font-mono text-[12px] opacity-0 group-hover:opacity-40 tracking-tighter transition-opacity duration-300"
            >
              ЧИТАТЬ →
            </span>
          </button>
          <button
            onclick={() => manager.deleteFile(file.file_name)}
            class="flex items-center justify-center bg-white/40 border border-white/10 rounded-2xl hover:bg-red-500 hover:text-white text-left
               w-0 opacity-0 pointer-events-none overflow-hidden hover:shadow-xl hover:-translate-y-0.5
               group-hover:w-14 group-hover:opacity-100 group-hover:pointer-events-auto
               transition-all duration-300 ease-out"
          >
            <span class="font-mono text-[12px] tracking-tighter"> X </span>
          </button>
        </div>
      {/each}
    </div>
  {/if}
  <Toast message={manager.toastMessage} isError={manager.isToastError} />
  <ConfirmDialog
    show={doc.showConfirmDelete}
    title="Удалить страницу?"
    message="Вы действительно хотите удалить страницу {doc.currentIndex +
      1}? Весь текст на ней будет безвозвратно стерт со всех дисков приложения."
    onConfirm={doc.confirmDeletion}
    onCancel={doc.cancelDeletion}
  />
</div>
