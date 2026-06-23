<script lang="ts">
  import { onMount, getContext, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Toast from "$lib/components/Toast.svelte";
  import type { Location } from "$lib/types/location";
  import "$lib/../app.css"; 

  interface NavState {
    isVisible: boolean;
  }
  let navState = getContext("nav") as NavState;

  const createEmptyLoc = (): Location => ({
    name: "",
    type_scene: "ИНТ.",
    time_day: "ДЕНЬ",
    lighting: "",
    interior_details: "",
    description: "",
  });

  let locations = $state<string[]>([]);
  let loc = $state<Location>(createEmptyLoc());
  let chooseFile = $state<string | null>(null);
  let newName = $state("");

  let toastMessage = $state<string | null>(null);
  let toastIsError = $state(false);
  let toastTimeout: number;

  function showToast(msg: string, isError = false) {
    clearTimeout(toastTimeout);
    toastMessage = msg;
    toastIsError = isError;
    toastTimeout = setTimeout(() => { toastMessage = null; }, 3000);
  }

  async function loadLocations() {
    try {
      locations = await invoke<string[]>("get_locations");
    } catch (e) {
      console.error("Ошибка загрузки локаций:", e);
    }
  }

  async function createNewLocation() {
    if (!newName.trim()) return;
    try {
      let tempLoc = createEmptyLoc();
      tempLoc.name = newName.trim();
      const fileName = await invoke<string>("create_location", { location: tempLoc });
      newName = "";
      await loadLocations();
      showToast(`Объект "${tempLoc.name}" добавлен в реестр`);
    } catch (e) {
      console.error(e);
      showToast("Ошибка при создании локации", true);
    }
  }

  async function openLocation(fileName: string) {
    try {
      loc = await invoke<Location>("read_location", { nameFile: fileName });
      chooseFile = fileName;
      if (navState) navState.isVisible = false;
    } catch (e) {
      console.error(e);
    }
  }

  async function saveLocation() {
    if (!chooseFile) return;
    try {
      const updatedFileName = await invoke<string>("write_to_location", {
        nameFile: chooseFile,
        location: loc
      });
      chooseFile = updatedFileName;
      showToast(`Данные объекта "${loc.name || 'Без названия'}" сохранены`);
    } catch (e) {
      console.error(e);
      showToast("Не удалось сохранить локацию", true);
    }
  }

  function closeLocation() {
    chooseFile = null;
    loc = createEmptyLoc();
    if (navState) navState.isVisible = true;
    loadLocations();
  }

  async function deleteLocation(fileName: string) {
    try {
      const res = await invoke<string>("delete_location_file", { nameFile: fileName });
      showToast(res);
      await loadLocations(); // Перезагружаем список
    } catch (e) {
      console.error(e);
      showToast("Не удалось удалить локацию", true);
    }
  }

  onMount(loadLocations);
  onDestroy(() => {
    if (navState) navState.isVisible = true;
  });
</script>

<div class="h-full p-5 font-serif text-paper">
  {#if chooseFile}
    <!-- ЭКРАН РЕДАКТОРА ЛОКАЦИИ -->
    <div class="flex h-full gap-8 animate-in fade-in duration-500">
      
      <!-- ЛЕВАЯ СТОРОНА: ФОРМА ВВОДА (В твоем стиле) -->
      <div class="flex-1 flex flex-col gap-6 overflow-y-auto pr-4">
        <header class="flex justify-between items-center border-b border-white/10 pb-4">
          <button onclick={closeLocation} class="font-mono text-md opacity-40 hover:opacity-100 transition-opacity">
            В РЕЕСТР ЛОКАЦИЙ
          </button>
          
          <!-- Умная шапка с обрезанием длинных названий -->
          <div class="text-center max-w-[350px] mx-auto">
            <h2 class="text-xl font-serif flex items-center justify-center gap-1.5 w-full">
              <span class="shrink-0">ОБЪЕКТ:</span>
              {#if loc.name}
                <span class="truncate max-w-[240px] inline-block align-bottom" title={loc.name}>{loc.name}</span>
              {:else}
                <span class="opacity-40">Новая локация</span>
              {/if}
            </h2>
            <span class="font-mono text-[14px] uppercase tracking-widest opacity-30">Параметры сцены</span>
          </div>

          <button
            onclick={saveLocation}
            class="bg-black text-white px-4 py-1 text-md font-mono uppercase rounded-sm hover:bg-white hover:text-black transition-colors"
          >
            Сохранить
          </button>
        </header>

        <!-- Название, Тип и Время -->
        <div class="grid grid-cols-4 gap-4">
          <div class="col-span-2 flex flex-col gap-2">
            <label for="loc-name" class="font-mono text-[15px] uppercase text-black/90">Название локации</label>
            <textarea
              id="loc-name"
              bind:value={loc.name}
              rows="1"
              oninput={(e) => { e.currentTarget.style.height = 'auto'; e.currentTarget.style.height = e.currentTarget.scrollHeight + 'px'; }}
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
              placeholder="Например: Квартира Зубека"
            ></textarea>
          </div>
          
          <div class="flex flex-col gap-2">
            <label for="type-scene" class="font-mono text-[15px] uppercase text-black/90">Тип</label>
            <select id="type-scene" bind:value={loc.type_scene} class="input-field bg-white rounded-md h-10 px-2 font-sans text-sm outline-none">
              <option value="ИНТ.">ИНТ. (Интерьер)</option>
              <option value="НАТ.">НАТ. (Натура)</option>
              <option value="ИНТ./НАТ.">ИНТ./НАТ.</option>
            </select>
          </div>

          <div class="flex flex-col gap-2">
            <label for="time-day" class="font-mono text-[15px] uppercase text-black/90">Время</label>
            <select id="time-day" bind:value={loc.time_day} class="input-field bg-white rounded-md h-10 px-2 font-sans text-sm outline-none">
              <option value="ДЕНЬ">ДЕНЬ</option>
              <option value="НОЧЬ">НОЧЬ</option>
              <option value="УТРО">УТРО</option>
              <option value="ВЕЧЕР">ВЕЧЕР</option>
            </select>
          </div>
        </div>

        <!-- Освещение и Детали интерьера -->
        <div class="grid grid-cols-3 gap-4">
          <div class="flex flex-col gap-2">
            <label for="lighting" class="font-mono text-[15px] uppercase text-black/90">Освещение</label>
            <textarea
              id="lighting"
              bind:value={loc.lighting}
              rows="1"
              oninput={(e) => { e.currentTarget.style.height = 'auto'; e.currentTarget.style.height = e.currentTarget.scrollHeight + 'px'; }}
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
              placeholder="Тусклый свет лампы, неон..."
            ></textarea>
          </div>
          
          <div class="col-span-2 flex flex-col gap-2">
            <label for="details" class="font-mono text-[15px] uppercase text-black/90">Важные детали / Предметы</label>
            <textarea
              id="details"
              bind:value={loc.interior_details}
              rows="1"
              oninput={(e) => { e.currentTarget.style.height = 'auto'; e.currentTarget.style.height = e.currentTarget.scrollHeight + 'px'; }}
              class="input-field bg-white rounded-md resize-none min-h-10 py-2.5 overflow-hidden"
              placeholder="Клеенчатая папка на столе, пельмени, пистолет..."
            ></textarea>
          </div>
        </div>

        <!-- Подробное описание -->
        <div class="flex flex-col gap-2 flex-1 min-h-75">
          <label for="description" class="font-mono text-[15px] uppercase text-black/90">Описание атмосферы / Референсы</label>
          <textarea
            id="description"
            bind:value={loc.description}
            class="input-field flex-1 resize-none p-3 font-serif text-sm leading-relaxed h-full min-h-62.5 bg-white rounded-md"
            placeholder="Опишите, как выглядит это место, какие звуки или общее настроение оно передает..."
          ></textarea>
        </div>
      </div>

      <!-- ПРАВАЯ СТОРОНА: КАРТОЧКА ПРЕДПРОСМОТРА ДОСЬЕ -->
      <div class="w-80 border-l border-white/10 pl-8 hidden lg:flex flex-col gap-6 shrink-0">
        <div class="p-6 border border-white/10 rounded-2xl bg-white/5 backdrop-blur-md sticky top-8 text-left">
          <div class="w-12 h-12 bg-white/10 rounded-full mb-4 mx-auto flex items-center justify-center text-2xl opacity-60">
            📍
          </div>
          <h3 class="text-center text-xl font-serif italic break-words leading-snug">
            {loc.type_scene} {loc.name || "Название объекта"} <br /> — {loc.time_day}
          </h3>

          <div class="mt-6 space-y-4 text-xs italic opacity-90 break-words border-t border-white/10 pt-4 max-h-[50vh] overflow-y-auto pr-1">
            {#if loc.lighting}
              <p>
                <span class="font-mono text-[14px] not-italic uppercase block mb-0.5">Свет:</span>
                {loc.lighting}
              </p>
            {/if}
            {#if loc.interior_details}
              <p>
                <span class="font-mono text-[14px] not-italic uppercase block mb-0.5">Важные детали:</span>
                {loc.interior_details}
              </p>
            {/if}
            {#if loc.description}
              <p>
                <span
                  class="font-mono text-[14px] not-italic text-black/90 uppercase block mb-1"
                  >Атмосфера места:</span
                >
                {loc.description}
              </p>
            {/if}
          </div>
        </div>
      </div>

    </div>
  {:else}
    <!-- ЭКРАН СПИСКА (РЕЕСТР ЛОКАЦИЙ) -->
    <header class="mb-10 flex justify-between items-end">
      <div class="relative pb-5">
        <h1 class="text-3xl italic">Локации</h1>
      </div>

      <div class="flex gap-2">
        <input
          bind:value={newName}
          placeholder="Название локации..."
          class="border-b border-black/10 bg-transparent px-2 text-sm outline-none"
          onkeydown={(e) => e.key === 'Enter' && createNewLocation()}
        />
        <button 
          onclick={createNewLocation} 
          class="border border-black/10 px-4 py-1 text-sm font-mono uppercase hover:bg-black hover:text-white transition-all"
        >
          + Добавить объект
        </button>
      </div>
    </header>
    
    <!-- СЕТКА КАРТОЧЕК ДОСТУПНЫХ ЛОКАЦИЙ -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each locations as file}
        <div class="group flex items-stretch gap-0 hover:gap-3 w-full transition-all duration-300">
          <button
            onclick={() => openLocation(file)}
            class="flex-1 flex items-center justify-between p-5 bg-white/40 border border-white/10 rounded-2xl hover:bg-white hover:shadow-xl hover:-translate-y-0.5 transition-all text-left"
          >
            <div class="flex items-center gap-4 overflow-hidden pr-2">
              <span class="text-xl truncate font-medium font-serif capitalize">
                {file.split(/[/\\]/).pop()?.replace(".writer", "").replace(/_/g, " ") || file}
              </span>
            </div>
            <span class="font-mono text-[12px] opacity-0 group-hover:opacity-60 tracking-wider transition-opacity shrink-0 uppercase">
              Объект →
            </span>
          </button>
          <button
            onclick={() => deleteLocation(file)}
            class="flex items-center justify-center bg-white/40 border border-white/10 rounded-2xl hover:bg-red-500 hover:text-white text-left
               w-0 opacity-0 pointer-events-none overflow-hidden hover:shadow-xl hover:-translate-y-0.5
               group-hover:w-14 group-hover:opacity-100 group-hover:pointer-events-auto
               transition-all duration-300 ease-out"
          >
            <span class="font-mono text-[12px] tracking-tighter font-bold"> X </span>
          </button>
        </div>
      {:else}
        <!-- СОСТОЯНИЕ ПУСТОГО РЕЕСТРА -->
        <div class="col-span-full py-20 text-center drounded-xl bg-white/[0.02]">
          <p class="font-mono text-xs opacity-30 tracking-widest">РЕЕСТР ЛОКАЦИЙ ПУСТ</p>
        </div>
      {/each}
    </div>
  {/if}

  <!-- КОМПОНЕНТ СИСТЕМНЫХ УВЕДОМЛЕНИЙ -->
  <Toast message={toastMessage} isError={toastIsError} />
</div>
