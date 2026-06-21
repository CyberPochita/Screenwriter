<script lang="ts">
  import { onMount, onDestroy, getContext } from "svelte";
  import { createDocumentStore } from "$lib/components/documentStore.svelte";
  import { createScenarioManager } from "$lib/scenario/scenarios.svelte";
  import { fade } from "svelte/transition";
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
    <div
      class="flex flex-col h-full animate-in fade-in duration-500 text-slate-900"
    >
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
          <span class="font-mono text-xs uppercase tracking-widest opacity-50"
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

      {#if doc.activeTab === "title"}
        <!-- ТИТУЛЬНЫЙ ЛИСТ С УЧЕТОМ СТРОГОЙ ТИПИЗАЦИИ -->
        <div
          class="flex-1 overflow-auto max-w-4xl mx-auto w-full p-6 bg-white border border-black/5 rounded-xl shadow-sm my-4"
        >
          <h3
            class="text-lg font-mono uppercase tracking-wider mb-6 pb-2 border-b border-gray-100"
          >
            Конфигурация титульной страницы
          </h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-8 font-sans">
            <!-- Левая сторона: Метаданные и Перечисления Rust -->
            <div class="space-y-4">
              <div>
                <label
                  for="title"
                  class="block text-xs font-bold uppercase tracking-wider text-gray-500 mb-1"
                  >Название фильма / сценария</label
                >
                <input
                  id="title"
                  bind:value={doc.titlePage.title}
                  type="text"
                  placeholder="ВВЕДИТЕ НАЗВАНИЕ"
                  class="w-full border border-gray-200 rounded px-3 py-2 text-sm uppercase font-mono focus:border-black focus:outline-none"
                />
              </div>

              <div>
                <label
                  for="author"
                  class="block text-xs font-bold uppercase tracking-wider text-gray-500 mb-1"
                  >Автор сценария</label
                >
                <input
                  id="author"
                  bind:value={doc.titlePage.author}
                  type="text"
                  placeholder="Имя Фамилия"
                  class="w-full border border-gray-200 rounded px-3 py-2 text-sm focus:border-black focus:outline-none"
                />
              </div>

              <!-- Управление Enum Authorship -->
              <div>
                <label
                  for="authorship"
                  class="block text-xs font-bold uppercase tracking-wider text-gray-500 mb-1"
                  >Тип авторства</label
                >
                <select
                  value={typeof doc.titlePage.authorship === "string"
                    ? "original"
                    : Object.keys(doc.titlePage.authorship)[0]}
                  onchange={(e) => {
                    const val = e.currentTarget.value;
                    if (val === "original") {
                      doc.titlePage.authorship = "original";
                    } else if (val === "adaptation") {
                      doc.titlePage.authorship = { adaptation: { source: "" } };
                    } else if (val === "basedOn") {
                      doc.titlePage.authorship = { basedOn: { source: "" } };
                    }
                  }}
                  class="w-full border border-gray-200 rounded px-3 py-2 text-sm bg-white focus:border-black focus:outline-none"
                >
                  <option value="original">Оригинальный сценарий</option>
                  <option value="adaptation">Экранизация (Adaptation)</option>
                  <option value="basedOn">Основано на (Based On)</option>
                </select>
              </div>

              <!-- Безопасный ввод для вложенных объектов структуры (Адаптации) -->
              {#if typeof doc.titlePage.authorship !== "string"}
                {@const currentAuth = doc.titlePage.authorship}
                <div>
                  <label
                    for="adaptation"
                    class="block text-xs font-bold uppercase tracking-wider text-gray-500 mb-1"
                    >Первоисточник</label
                  >
                  <input
                    id="adaptation"
                    value={"adaptation" in currentAuth
                      ? currentAuth.adaptation.source
                      : currentAuth.basedOn.source}
                    oninput={(e) => {
                      const value = e.currentTarget.value;
                      const current = doc.titlePage.authorship;

                      if (typeof current === "object" && current !== null) {
                        if ("adaptation" in current) {
                          // Атомарно перезаписываем объект, сохраняя структуру
                          doc.titlePage.authorship = {
                            adaptation: { source: value },
                          };
                        } else if ("basedOn" in current) {
                          // Атомарно перезаписываем объект, сохраняя структуру
                          doc.titlePage.authorship = {
                            basedOn: { source: value },
                          };
                        }
                      }
                    }}
                    type="text"
                    placeholder="Роман / Комикс / Телепередача"
                    class="w-full border border-gray-200 rounded px-3 py-2 text-sm focus:border-black focus:outline-none"
                  />
                </div>
              {/if}
            </div>

            <!-- Правая сторона: Контакты (с поддержкой null) -->
            <div class="space-y-4">
              <div class="flex items-center gap-2 pb-2">
                <input
                  type="checkbox"
                  id="agentMode"
                  checked={doc.titlePage.contact.agent !== null &&
                    doc.titlePage.contact.agent !== undefined}
                  onchange={(e) => {
                    if (e.currentTarget.checked) {
                      // Инициализируем агента, обнуляем личное имя
                      doc.titlePage.contact.agent = {
                        name: "",
                        company: "",
                        phone: "",
                        email: "",
                      };
                      doc.titlePage.contact.name = "";
                    } else {
                      // Удаляем агента
                      doc.titlePage.contact.agent = null;
                    }
                  }}
                  class="rounded border-gray-300 accent-black text-black"
                />
                <label
                  for="agentMode"
                  class="text-sm font-medium text-gray-700 select-none"
                  >Указать контакты агента</label
                >
              </div>

              {#if !doc.titlePage.contact.agent}
                <!-- Режим личных контактов -->
                <div
                  class="space-y-3 p-3 bg-gray-50 rounded-lg border border-gray-100"
                >
                  <div>
                    <label
                      for="nameContact"
                      class="block text-[10px] font-bold text-gray-400 uppercase"
                      >Имя контакта</label
                    >
                    <input
                      id="nameContact"
                      bind:value={doc.titlePage.contact.name}
                      type="text"
                      class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                    />
                  </div>
                  <div>
                    <label
                      for="email"
                      class="block text-[10px] font-bold text-gray-400 uppercase"
                      >Почтовый адрес</label
                    >
                    <input
                      id="email"
                      value={doc.titlePage.contact.address ?? ""}
                      oninput={(e) =>
                        (doc.titlePage.contact.address =
                          e.currentTarget.value || null)}
                      type="text"
                      class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                    />
                  </div>
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label
                        for="phone"
                        class="block text-[10px] font-bold text-gray-400 uppercase"
                        >Телефон</label
                      >
                      <input
                        id="phone"
                        value={doc.titlePage.contact.phone ?? ""}
                        oninput={(e) =>
                          (doc.titlePage.contact.phone =
                            e.currentTarget.value || null)}
                        type="text"
                        class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                      />
                    </div>
                    <div>
                      <label
                        for="email2"
                        class="block text-[10px] font-bold text-gray-400 uppercase"
                        >Email</label
                      >
                      <input
                        id="email2"
                        value={doc.titlePage.contact.email ?? ""}
                        oninput={(e) =>
                          (doc.titlePage.contact.email =
                            e.currentTarget.value || null)}
                        type="text"
                        class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                      />
                    </div>
                  </div>
                </div>
              {:else}
                <!-- Режим агента: так как агент гарантированно инициализирован в блоке if, биндинг безопасен -->
                <div
                  class="space-y-3 p-3 bg-gray-50 rounded-lg border border-black/10"
                >
                  <div>
                    <label
                      for="fio"
                      class="block text-[10px] font-bold text-gray-800 uppercase"
                      >ФИО Агента</label
                    >
                    <input
                      id="fio"
                      bind:value={doc.titlePage.contact.agent.name}
                      type="text"
                      class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                    />
                  </div>
                  <div>
                    <label
                      for="company"
                      class="block text-[10px] font-bold text-gray-800 uppercase"
                      >Агентство / Компания</label
                    >
                    <input
                      id="company"
                      bind:value={doc.titlePage.contact.agent.company}
                      type="text"
                      class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                    />
                  </div>
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label
                        for="phoneAgent"
                        class="block text-[10px] font-bold text-gray-800 uppercase"
                        >Телефон агента</label
                      >
                      <input
                        id="phoneAgent"
                        bind:value={doc.titlePage.contact.agent.phone}
                        type="text"
                        class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                      />
                    </div>
                    <div>
                      <label
                        for="emailAgent"
                        class="block text-[10px] font-bold text-gray-800 uppercase"
                        >Email агента</label
                      >
                      <input
                        id="emailAgent"
                        bind:value={doc.titlePage.contact.agent.email}
                        type="text"
                        class="w-full bg-white border border-gray-200 rounded px-3 py-1.5 text-sm focus:border-black focus:outline-none"
                      />
                    </div>
                  </div>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {:else}
        <!-- СТАНДАРТНЫЙ ЭКРАН РЕДАКТОРА ТЕКСТА (Ваш оригинальный UI) -->
        <div
          class="flex justify-center items-center w-full h-screen overflow-auto font-mono text-lg"
        >
          <div class="relative">
            <Editor bind:value={doc.currentPage.text} onAddPage={doc.addPage} />
            <div
              class="absolute top-4 left-[calc(100%+16px)] flex flex-col gap-3 w-40 min-h-205"
            >
              <div class="flex flex-col gap-1 text-black/70">
                <p class="text-sm text-black/50 select-none">Страницы</p>
                <p class="text-lg text-black/50 font-bold select-none">
                  {doc.currentIndex + 1}/{doc.pages.length}
                </p>
              </div>

              <div class="mt-auto flex flex-col gap-3">
                <button
                  onclick={() => doc.addPage()}
                  class="w-full py-2.5 px-4 bg-white border border-black/10 text-black/70 rounded-xl font-sans text-sm font-semibold hover:bg-black hover:text-white hover:shadow-xl transition-all cursor-pointer text-center"
                >
                  Создать страницу
                </button>

                <div class="flex gap-1 h-11">
                  <button
                    onclick={doc.prev}
                    disabled={doc.currentIndex === 0}
                    class="flex-1 flex items-center justify-center p-3 bg-white text-black/50 border border-black/10 rounded-2xl hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer"
                  >
                    ❮
                  </button>
                  <button
                    onclick={doc.next}
                    disabled={doc.currentIndex === doc.pages.length - 1}
                    class="flex-1 flex items-center justify-center p-3 bg-white border border-black/10 rounded-2xl text-black/50 hover:bg-black hover:text-white disabled:opacity-30 disabled:pointer-events-none transition-all cursor-pointer"
                  >
                    ❯
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    ...

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
          <!-- Основная кнопка -->
          <button
            onclick={() => manager.loadContent(file.file_name)}
            class="flex-1 flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all text-left"
          >
            <span class="text-xl text-writer-dark/80">{file.file_name}</span>
            {#if file.file_format === ""}
              <span class="text-xl text-writer-dark/80">Проект</span>
            {/if}
            <span
              class="font-mono text-[10px] opacity-0 group-hover:opacity-40 tracking-tighter transition-opacity duration-300"
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
</div>
