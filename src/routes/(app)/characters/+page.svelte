<script lang="ts">
  import { onMount, getContext, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Character } from "$lib/types/character";
  import "../../../app.css";
  import Toast from "$lib/components/Toast.svelte";

  // Контекст для скрытия меню навигации
  interface NavState {
    isVisible: boolean;
  }
  let navState = getContext("nav") as NavState;
  let toastMessage = $state<string | null>(null);
  let toastIsError = $state(false);
  let toastTimeout: number;

  // Инициализация пустой анкеты
  const createEmptyChar = (): Character => ({
    first_name: "",
    last_name: "",
    middle_name: "",
    age: 0,
    habits: "",
    likes: "",
    dislikes: "",
    description: "",
  });

  let characters = $state<string[]>([]);
  let char = $state<Character>(createEmptyChar());

  let chooseFile = $state<string | null>(null); // Имя редактируемого .json файла
  let newName = $state(""); // Для быстрого создания

  function showToast(msg: string, isError = false) {
    clearTimeout(toastTimeout); // Сбрасываем предыдущий таймер
    toastMessage = msg;
    toastIsError = isError;

    // Скрываем тост через 3 секунды
    toastTimeout = setTimeout(() => {
      toastMessage = null;
    }, 3000);
  }

  async function loadCharacters() {
    try {
      characters = await invoke("get_characters");
    } catch (e) {
      console.error("Failed to load characters:", e);
    }
  }

  function autoResize(node: HTMLTextAreaElement) {
    const update = () => {
      node.style.height = "auto";
      node.style.height = `${node.scrollHeight}px`;
    };
    update();
    node.addEventListener("input", update);
    return {
      destroy() {
        node.removeEventListener("input", update);
      },
    };
  }

  async function createNewCharacter() {
    if (!newName) return;
    try {
      // Инициализируем файл пустой структурой с именем
      let tempChar = createEmptyChar();
      tempChar.last_name = newName;
      console.log(tempChar.last_name);
      const fileName = `${newName}.json`;
      await invoke("create_character", { character: tempChar }); // Передаем структуру, Rust сам сделает файл
      newName = "";
      await loadCharacters();
    } catch (e) {
      console.error(e);
    }
  }

  async function openCharacter(fileName: string) {
    try {
      console.log(fileName);
      char = await invoke("read_character", { nameFile: fileName });
      chooseFile = fileName;
      navState.isVisible = false; // Прячем навигацию
    } catch (e) {
      console.error(e);
    }
  }

  async function saveCharacter() {
    if (!chooseFile) {
      showToast("Нет открытого файла для сохранения", true);
      return;
    }
    try {
      // ИСПРАВЛЕНО: Ловим строку с актуальным именем файла от Rust
      const updatedFileName = await invoke<string>("write_to_character", {
        nameFile: chooseFile,
        character: char,
      });

      // Обновляем состояние фронтенда новым именем файла
      chooseFile = updatedFileName;

      showToast(
        `Изменения в деле "${char.last_name || "Без фамилии"}" сохранены`,
      );
    } catch (e) {
      console.error(e);
      showToast("Не удалось сохранить файл", true);
    }
  }

  function closeCharacter() {
    chooseFile = null;
    char = createEmptyChar();
    navState.isVisible = true; // Возвращаем навигацию
    loadCharacters();
  }

  onMount(loadCharacters);
  onDestroy(() => {
    navState.isVisible = true;
  });
</script>

<div class="h-full p-5 font-serif">
  {#if chooseFile}
    <!-- ЭКРАН РЕДАКТОРА АНКЕТЫ -->
    <div class="flex h-full gap-8 animate-in fade-in duration-500">
      <!-- ЛЕВАЯ СТОРОНА: ФОРМА ВВОДА -->
      <div class="flex-1 flex flex-col gap-6 overflow-y-auto pr-4">
        <header
          class="flex justify-between items-center border-b border-white/5 pb-4"
        >
          <button
            onclick={closeCharacter}
            class="font-mono text-md opacity-40 hover:opacity-100 transition-opacity"
          >
            В АРХИВ ЛИЧНОСТЕЙ
          </button>
          <div class="text-center">
            <h2
              class="text-xl font-serif flex items-center justify-center gap-1.5 w-full"
            >
              <span class="shrink-0">ДЕЛО:</span>

              {#if char.first_name || char.last_name || char.middle_name}
                <!-- Фамилия -->
                {#if char.last_name}
                  <span
                    class="truncate max-w-50 inline-block align-bottom"
                    title={char.last_name}
                  >
                    {char.last_name}
                  </span>
                {/if}

                <!-- Имя -->
                {#if char.first_name}
                  <span
                    class="truncate max-w-50 inline-block align-bottom"
                    title={char.first_name}
                  >
                    {char.first_name}
                  </span>
                {/if}

                <!-- Отчество -->
                {#if char.middle_name}
                  <span
                    class="truncate max-w-50 inline-block align-bottom"
                    title={char.middle_name}
                  >
                    {char.middle_name}
                  </span>
                {/if}
              {:else}
                <span class="opacity-40">Новый персонаж</span>
              {/if}
            </h2>
            <span
              class="font-mono text-[14px] uppercase tracking-widest opacity-30"
              >Редактирование параметров</span
            >
          </div>
          <button
            onclick={saveCharacter}
            class="bg-black text-white px-4 py-1 text-md font-mono uppercase rounded-sm hover:bg-white hover:text-black transition-colors"
          >
            Сохранить
          </button>
        </header>

        <div class="grid grid-cols-3 gap-4">
          <div class="flex flex-col gap-2">
            <label
              for="first-name"
              class="font-mono text-[15px] uppercase text-black/90">Имя</label
            >
            <textarea
              id="first-name"
              bind:value={char.first_name}
              use:autoResize
              rows="1"
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
            ></textarea>
          </div>

          <div class="flex flex-col gap-2">
            <label
              for="last-name"
              class="font-mono text-[15px] uppercase text-black/90"
              >Фамилия</label
            >
            <textarea
              id="last-name"
              bind:value={char.last_name}
              use:autoResize
              rows="1"
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
            ></textarea>
          </div>

          <div class="flex flex-col gap-2">
            <label
              for="middle-name"
              class="font-mono text-[15px] uppercase text-black/90"
              >Отчество</label
            >
            <textarea
              id="middle-name"
              bind:value={char.middle_name}
              use:autoResize
              rows="1"
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
            ></textarea>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4">
          <div class="flex flex-col gap-2">
            <label
              for="age"
              class="font-mono text-[15px] uppercase text-black/90"
              >Возраст</label
            >
            <input
              id="age"
              type="number"
              min="0"
              max="150"
              value={char.age || 0}
              oninput={(e) => (char.age = parseInt(e.currentTarget.value) || 0)}
              class="input-field bg-white rounded-md h-10"
            />
          </div>

          <div class="col-span-3 flex flex-col gap-2">
            <label
              for="habits"
              class="font-mono text-[15px] uppercase text-black/90"
              >Привычки</label
            >
            <textarea
              id="habits"
              bind:value={char.habits}
              use:autoResize
              rows="1"
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
              placeholder="Особые приметы, жесты, мимика..."
            ></textarea>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-2">
            <label
              for="likes"
              class="font-mono text-[15px] uppercase tracking-widest text-black/90"
              >Любит</label
            >
            <textarea
              id="likes"
              bind:value={char.likes}
              class="input-field h-24 resize-none bg-white rounded-md p-3"
            ></textarea>
          </div>

          <div class="flex flex-col gap-2">
            <label
              for="dislikes"
              class="font-mono text-[15px] uppercase tracking-widest text-black/90"
              >Не любит</label
            >
            <textarea
              id="dislikes"
              bind:value={char.dislikes}
              class="input-field h-24 resize-none bg-white rounded-md p-3"
            ></textarea>
          </div>
        </div>

        <div class="flex flex-col gap-2 flex-1 min-h-75">
          <label
            for="description"
            class="font-mono text-[15px] uppercase text-black/90"
            >Подробная биография / Психотип</label
          >
          <textarea
            id="description"
            bind:value={char.description}
            class="input-field flex-1 resize-none p-3 font-serif text-sm leading-relaxed h-full min-h-62.5 bg-white rounded-md"
          ></textarea>
        </div>
      </div>

      <!-- ПРАВАЯ СТОРОНА: ПРЕДПРОСМОТР КАРТОЧКИ (Не ломает границы) -->
      <div
        class="w-80 border-l border-white/10 pl-8 hidden lg:flex flex-col gap-6"
      >
        <div
          class="p-6 border border-white/10 rounded-2xl bg-white/5 backdrop-blur-md overflow-hidden"
        >
          <div
            class="w-16 h-16 bg-white/10 rounded-full mb-4 mx-auto flex items-center justify-center text-2xl opacity-20"
          >
            👤
          </div>
          <h3
            class="text-center text-xl italic wrap-break-word leading-tight text-black/90"
          >
            {char.first_name || "Имя"} <br />
            {char.last_name} <br />
            {char.middle_name}
          </h3>
          <p
            class="text-center font-mono text-[14px] opacity-60 uppercase mt-2 text-black"
          >
            {char.age ? char.age + " лет" : "Возраст не указан"}
          </p>

          <div
            class="mt-6 space-y-4 text-xs italic opacity-80 wrap-break-word border-t border-white/5 pt-4"
          >
            {#if char.habits}
              <p>
                <span
                  class="font-mono text-[14px] not-italic uppercase block mb-1 text-black/90"
                  >Привычка:</span
                >
                {char.habits}
              </p>
            {/if}
            {#if char.likes}
              <p>
                <span
                  class="font-mono text-[14px] not-italic text-black/90 uppercase block mb-1"
                  >Любит:</span
                >
                {char.likes}
              </p>
            {/if}
            {#if char.dislikes}
              <p>
                <span
                  class="font-mono text-[14px] not-italic text-black/90 uppercase block mb-1"
                  >Не любит:</span
                >
                {char.dislikes}
              </p>
            {/if}
            {#if char.description}
              <p>
                <span
                  class="font-mono text-[14px] not-italic text-black/90 uppercase block mb-1"
                  >Подробная биография:</span
                >
                {char.description}
              </p>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (АРХИВ ПЕРСОНАЖЕЙ) -->
    <header class="mb-10 flex justify-between items-end">
      <div class="relative pb-5">
        <h1 class="text-3xl italic">Персонажи</h1>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={newName}
          placeholder="Фамилия нового героя..."
          class="border-b border-black/10 bg-transparent px-2 text-sm outline-none"
        />
        <button
          onclick={createNewCharacter}
          class="border border-black/10 px-4 py-1 text-sm font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать анкету
        </button>
      </div>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
      {#each characters as file}
        <button
          onclick={() => openCharacter(file)}
          class="flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all group text-left"
        >
          <div class="flex items-center gap-4 overflow-hidden">
            <span class="text-xl truncate font-medium font-serif capitalize">
              {file
                .split(/[/\\]/)
                .pop()
                ?.replace(".writer", "")
                .replace(/_/g, " ") || file}
            </span>
          </div>
          <span
            class="font-mono text-[10px] opacity-0 group-hover:opacity-60 tracking-tighter shrink-0"
            >ДОСЬЕ →</span
          >
        </button>
      {:else}
        <div
          class="col-span-full py-20 text-center border-2 border-dashed border-white/5 rounded-2xl"
        >
          <p class="font-mono text-xs opacity-20">СПИСОК ПЕРСОНАЖЕЙ ПУСТ</p>
        </div>
      {/each}
    </div>
  {/if}
  <Toast message={toastMessage} isError={toastIsError} />
</div>
