<script lang="ts">
  import { onMount, getContext, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Editor from "$lib/components/Editor.svelte";
  import type { Character } from "$lib/types/character";
  import "../../../app.css";

  // Контекст для скрытия меню навигации
  interface NavState {
    isVisible: boolean;
  }
  let navState = getContext("nav") as NavState;

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

  async function loadCharacters() {
    try {
      characters = await invoke("get_characters");
    } catch (e) {
      console.error("Failed to load characters:", e);
    }
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
    try {
      // Передаем обновленные данные. Бэкенд перезапишет файл
      await invoke("write_to_character", { character: char });
    } catch (e) {
      console.error(e);
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

<div class="h-full p-8 font-serif text-paper">
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
            class="font-mono text-xs opacity-40 hover:opacity-100 transition-opacity"
          >
            ← В АРХИВ ЛИЧНОСТЕЙ
          </button>
          <div class="text-center">
            <h2 class="text-xl italic">
              Дело: {char.first_name}
              {char.last_name}
            </h2>
            <span
              class="font-mono text-[12px] uppercase tracking-widest opacity-30"
              >Редактирование параметров</span
            >
          </div>
          <button
            onclick={saveCharacter}
            class="bg-white text-black px-4 py-1 text-xs font-mono uppercase rounded-sm hover:bg-white/80 transition-colors"
          >
            Сохранить
          </button>
        </header>

        <div class="grid grid-cols-3 gap-4">
          <div class="flex flex-col gap-2">
            <label for="first-name" class="font-mono text-[13px] uppercase text-black/90"
              >Имя</label
            >
            <input
              id="first-name"
              bind:value={char.first_name}
              class="input-field"
              placeholder="Имя"
            />
          </div>
          <div class="flex flex-col gap-2">
            <label for="last-name" class="font-mono text-[13px] uppercase text-black/90"
              >Фамилия</label
            >
            <input
              id="last-name"
              bind:value={char.last_name}
              class="input-field"
              placeholder="Фамилия"
            />
          </div>
          <div class="flex flex-col gap-2">
            <label for="middle-name" class="font-mono text-[13px] uppercase text-black/90"
              >Отчество</label
            >
            <input
              id="middle-name"
              bind:value={char.middle_name}
              class="input-field"
              placeholder="Отчество"
            />
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4">
          <div class="flex flex-col gap-2">
            <label for="age" class="font-mono text-[13px] uppercase text-black/90"
              >Возраст</label
            >
            <input
              id="age"
              type="number"
              bind:value={char.age}
              class="input-field"
              placeholder="Лет"
            />
          </div>
          <div class="col-span-3 flex flex-col gap-2">
            <label for="habits" class="font-mono text-[13px] uppercase text-black/90"
              >Привычки</label
            >
            <input
              id="habits"
              bind:value={char.habits}
              class="input-field"
              placeholder="Особые приметы, жесты, мимика..."
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-2">
            <label
              for="likes"
              class="font-mono text-[13px] uppercase tracking-widest text-black/90"
              >Любит</label
            >
            <textarea
              id="likes"
              bind:value={char.likes}
              class="input-field h-24 resize-none"
            ></textarea>
          </div>
          <div class="flex flex-col gap-2">
            <label
              for="dislikes"
              class="font-mono text-[13px] uppercase tracking-widest text-black/90"
              >Не любит</label
            >
            <textarea
              id="dislikes"
              bind:value={char.dislikes}
              class="input-field h-24 resize-none"
            ></textarea>
          </div>
        </div>

        <div class="flex flex-col gap-2 flex-1 min-h-75">
          <label
            for="description"
            class="font-mono text-[13px] uppercase text-black/90"
            >Подробная биография / Психотип</label
          >
          <!-- Твой переиспользуемый CodeMirror компонент -->
          <Editor bind:value={char.description} />
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
          <h3 class="text-center text-xl italic wrap-break-word leading-tight text-black/90">
            {char.first_name || "Имя"} <br />
            {char.last_name} <br />
            {char.middle_name}
          </h3>
          <p
            class="text-center font-mono text-[12px] opacity-40 uppercase mt-2 text-black"
          >
            {char.age ? char.age + " лет" : "Возраст не указан"}
          </p>

          <div
            class="mt-6 space-y-4 text-xs italic opacity-80 wrap-break-word border-t border-white/5 pt-4"
          >
            {#if char.habits}
              <p>
                <span
                  class="font-mono text-[12px] not-italic opacity-40 uppercase block mb-1 text-black/90"
                  >Привычка:</span
                >
                {char.habits}
              </p>
            {/if}
            {#if char.likes}
              <p>
                <span
                  class="font-mono text-[12px] not-italic text-black/90 uppercase block mb-1"
                  >Любит:</span
                >
                {char.likes}
              </p>
            {/if}
            {#if char.dislikes}
              <p>
                <span
                  class="font-mono text-[12px] not-italic text-black/90 uppercase block mb-1"
                  >Не любит:</span
                >
                {char.dislikes}
              </p>
            {/if}
            {#if char.description}
              <p>
                <span
                  class="font-mono text-[12px] not-italic text-black/90 uppercase block mb-1"
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
      <div>
        <h1 class="text-3xl italic text-writer-focus">Действующие лица</h1>
        <p
          class="font-mono text-[10px] opacity-40 mt-2 uppercase tracking-widest"
        >
          Scriptwriter_OS // Personas_Database
        </p>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={newName}
          placeholder="Имя нового героя..."
          class="border-b border-black/10 bg-transparent px-2 text-xs outline-none"
        />
        <button
          onclick={createNewCharacter}
          class="border border-black/10 px-4 py-1 text-xs font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Создать анкету
        </button>
      </div>
    </header>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each characters as file}
        <button
          onclick={() => openCharacter(file)}
          class="flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all group text-left"
        >
          <div class="flex items-center gap-4 overflow-hidden">
            <!-- Отрезаем расширение .json для красоты -->
            <span class="text-lg truncate font-medium"
              >{file.replace(".json", "")}</span
            >
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
</div>
