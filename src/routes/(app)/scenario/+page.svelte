<script lang="ts">
  import { onMount, onDestroy, getContext } from "svelte";
  import { createDocumentStore } from "$lib/components/documentStore.svelte";
  import { createScenarioManager } from "$lib/scenario/scenarios.svelte";

  import "$lib/components/EditPanel.svelte";
  import Editor from "$lib/components/Editor.svelte";

  interface NavState {
    isVisible: boolean;
  }

  let navState = getContext("nav") as NavState;
  let editorSettings = getContext<any>("editor-settings");
  const doc = createDocumentStore();
  const manager = createScenarioManager(navState, doc);

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
    <!-- ЭКРАН РЕДАКТОРА -->
    <div class="flex flex-col h-full animate-in fade-in duration-500">
      <header
        class="mb-6 flex justify-between items-center border-b border-black/5 pb-4"
      >
        <button
          onclick={manager.closeFile}
          class="font-mono text-sm opacity-40 hover:opacity-100 transition-opacity"
        >
          ← ВЕРНУТЬСЯ В АРХИВ
        </button>
        <div class="text-center">
          <h2 class="text-xl italic">{manager.chooseFile}</h2>
          <span
            class="font-mono text-xs uppercase tracking-widest opacity-50"
            >Режим редактирования</span
          >
        </div>
        <button
          onclick={manager.saveContent}
          class="bg-black text-white px-4 py-1 text-sm font-mono uppercase rounded-sm hover:bg-gray-800 transition-colors"
        >
          Сохранить
        </button>
      </header>

      <!-- Само содержимое файла -->
      <div
        class="flex justify-center items-center w-full h-screen overflow-auto font-mono text-lg"
      >
        <div class="relative">
          <Editor bind:value={doc.currentPage.text} onAddPage={doc.addPage} />
          <div
            class="absolute top-4 left-[calc(100%+16px)] flex flex-col gap-3 w-40 min-h-205"
          >
            <!-- Этот блок останется наверху -->
            <div class="flex flex-col gap-1 text-white/70">
              <p class="text-sm text-black/50 select-none">Страницы</p>
              <p class="text-lg text-black/50 font-bold select-none">
                {doc.currentIndex + 1}/{doc.pages.length}
              </p>
            </div>

            <!-- НОВЫЙ КОНТЕЙНЕР: mt-auto унесет всё содержимое вниз, а flex-col и gap-3 сохранят отступы между кнопками -->
            <div class="mt-auto flex flex-col gap-3">
              <!-- Кнопка создания новой страницы вручную -->
              <button
                onclick={() => doc.addPage()}
                class="w-full py-2.5 px-4 bg-white/10 border border-white/10 text-black/50 rounded-xl font-sans text-sm font-semibold hover:bg-black hover:text-white hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0 transition-all cursor-pointer text-center"
              >
                Создать страницу
              </button>

              <!-- Стрелочки перелистывания -->
              <div class="flex gap-1 h-11">
                <button
                  onclick={doc.prev}
                  disabled={doc.currentIndex === 0}
                  class="flex-1 flex items-center justify-center p-3 bg-white/10 text-black/50 border border-white/10 rounded-2xl hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer"
                >
                  ❮
                </button>
                <button
                  onclick={doc.next}
                  disabled={doc.currentIndex === doc.pages.length - 1}
                  class="flex-1 flex items-center justify-center p-3 bg-white/10 border border-white/10 rounded-2xl text-black/50 hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer"
                >
                  ❯
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (АРХИВ) -->
    <header class="mb-10 flex justify-between items-end">
      <div>
        <h1 class="text-3xl italic">Сценарии</h1>
        {#if manager.currentProject !== "scenarios"}
          <button
            onclick={manager.exitProject}
            class="font-mono text-sm opacity-40 hover:opacity-100 transition-opacity"
          >
            ← НАЗАД
          </button>
        {/if}
      </div>

      <div class="">
        <p class="font-mono text-xl">
          {#if manager.currentProject}
            Папка: <span class="italic">{manager.currentProject}</span>
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
          <!-- Основная кнопка -->
          <button
            onclick={() => manager.loadContent(file)}
            class="flex-1 flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all text-left"
          >
            <span class="text-xl text-writer-dark/80">{file}</span>
            <span
              class="font-mono text-[10px] opacity-0 group-hover:opacity-40 tracking-tighter transition-opacity duration-300"
            >
              ЧИТАТЬ →
            </span>
          </button>
          <button
            onclick={() => manager.deleteFile(file)}
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
</div>
