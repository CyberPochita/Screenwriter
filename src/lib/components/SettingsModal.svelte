<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog"; // Нативный диалог выбора папок
  import { fade, scale } from "svelte/transition";

  // Принимаем стейт видимости из контекста Svelte 5
  let { show = $bindable(false), onNotify = (msg: string) => {} } = $props();

  // Локальное состояние настроек
  let mainDir = $state("");
  let scenariosDir = $state("");
  let charactersDir = $state("");
  let locationsDir = $state("");
  
  // Дополнительные крутые фичи для сценариста
  let autoSaveInterval = $state(60); // в секундах
  let themeMode = $state("paper"); // paper, dark, sepia
  let exportFormat = $state("pdf"); // pdf, fdx, txt

  // Загружаем актуальные настройки из Rust при открытии окна
  async function loadSettings() {
    try {
      const opts = await invoke<any>("get_app_options");
      mainDir = opts.main_dir;
      scenariosDir = opts.scenarios_dir;
      charactersDir = opts.characters_dir;
      locationsDir = opts.locations_dir; // 🌟 ДОБАВЛЕНО
    } catch (e) {
      console.error(e);
    }
  }

  // Функция вызова нативного окна выбора папки на ПК
  async function selectDirectory(type: 'scenarios' | 'characters') {
    const selected = await open({
      directory: true, // Говорим, что ищем именно папку, а не файл
      multiple: false,
      title: "Выберите директорию для сохранения файлов проекта"
    });

    if (selected && typeof selected === "string") {
      if (type === 'scenarios') scenariosDir = selected;
      if (type === 'characters') charactersDir = selected;
    }
  }

  // Сохраняем измененные данные обратно в Rust config.json
  async function saveSettings() {
    try {
      const newOptions = {
        main_dir: mainDir,
        scenarios_dir: scenariosDir,
        characters_dir: charactersDir,
        locations_dir: locationsDir, // 🌟 ДОБАВЛЕНО
        current_dir: scenariosDir,
      };
      const res = await invoke<string>("save_app_options", { newOpts: newOptions });
      onNotify(res);
      show = false;
    } catch (e) {
      console.error(e);
    }
  }

  onMount(loadSettings);
</script>

{#if show}
  <!-- ЗАДНИЙ ФОН (Оверлей с размытием) -->
  <div 
    transition:fade={{ duration: 200 }}
    class="fixed inset-0 z-[9999] bg-black/40 backdrop-blur-md flex items-center justify-center p-4 select-none"
  >
    <!-- ОКНО НАСТРОЕК -->
    <div 
      transition:scale={{ duration: 250, start: 0.95 }}
      class="w-156.25 bg-white rounded-2xl border border-black/10 shadow-2xl flex flex-col overflow-hidden font-sans text-left text-slate-800"
    >
      <!-- ШАПКА ОКНА -->
      <header class="p-5 border-b border-black/5 flex justify-between items-center bg-gray-50/50">
        <div>
          <h3 class="text-lg font-bold font-mono">Параметры Системы</h3>
        </div>
      </header>

      <!-- ТЕЛО НАСТРОЕК (Скролл-бокс) -->
      <div class="p-6 flex flex-col gap-5 overflow-y-auto max-h-[70vh]">
        
        <!-- БЛОК 1: ДИРЕКТОРИИ ФАЙЛОВ -->
        <div class="flex flex-col gap-3">
          <h4 class="font-mono text-[12px] uppercase tracking-wider text-black/40 font-bold border-b border-black/5 pb-1">
            Пути хранения данных
          </h4>
          
          <!-- Папка сценариев -->
          <div class="flex flex-col gap-1.5">
            <label class="font-mono text-sm text-black/60 uppercase">Папка сценариев и проектов</label>
            <div class="flex gap-2">
              <input readonly value={scenariosDir} class="flex-1 h-9 px-3 border border-black/10 rounded-lg bg-gray-50 font-mono text-xs outline-none truncate" />
              <button onclick={() => selectDirectory('scenarios')} class="h-9 px-3 border border-black/10 bg-white rounded-lg font-mono text-md hover:bg-black hover:text-white transition-all active:scale-95">
                Обзор
              </button>
            </div>
          </div>

          <!-- Папка персонажей -->
          <div class="flex flex-col gap-1.5 mt-1">
            <label class="font-mono text-sm text-black/60 uppercase">Папка персонажей</label>
            <div class="flex gap-2">
              <input readonly value={charactersDir} class="flex-1 h-9 px-3 border border-black/10 rounded-lg bg-gray-50 font-mono text-xs outline-none truncate" />
              <button onclick={() => selectDirectory('characters')} class="h-9 px-3 border border-black/10 bg-white rounded-lg font-mono text-md hover:bg-black hover:text-white transition-all active:scale-95">
                Обзор
              </button>
            </div>
          </div>

          <div class="flex flex-col gap-1.5 mt-1">
            <label class="font-mono text-sm text-black/60 uppercase">Папка локаций</label>
            <div class="flex gap-2">
              <input readonly value={locationsDir} class="flex-1 h-9 px-3 border border-black/10 rounded-lg bg-gray-50 font-mono text-xs outline-none truncate" />
              <button onclick={() => selectDirectory('characters')} class="h-9 px-3 border border-black/10 bg-white rounded-lg font-mono text-md hover:bg-black hover:text-white transition-all active:scale-95">
                Обзор
              </button>
            </div>
          </div>
        </div>

        <!-- <div class="grid grid-cols-2 gap-4 mt-2">
          <div class="flex flex-col gap-1.5">
            <label for="autosave" class="font-mono text-xs text-black/60 uppercase">Автосохранение текста</label>
            <select id="autosave" bind:value={autoSaveInterval} class="h-9 px-2 border border-black/10 bg-white rounded-lg font-sans text-xs outline-none">
              <option value={15}>Каждые 15 секунд</option>
              <option value={30}>Каждые 30 секунд</option>
              <option value={60}>Каждую минуту</option>
              <option value={300}>Каждые 5 минут</option>
              <option value={0}>Отключить (Не рекомендуется)</option>
            </select>
          </div>

          <div class="flex flex-col gap-1.5">
            <label for="export" class="font-mono text-xs text-black/60 uppercase">Формат экспорта по умолчанию</label>
            <select id="export" bind:value={exportFormat} class="h-9 px-2 border border-black/10 bg-white rounded-lg font-sans text-xs outline-none">
              <option value="pdf">Кинематографический PDF</option>
              <option value="fdx">Final Draft (.fdx)</option>
              <option value="txt">Чистый текст (.txt)</option>
            </select>
          </div>
        </div>

        <div class="flex flex-col gap-2 mt-2">
          <label class="font-mono text-xs text-black/60 uppercase">Атмосфера рабочего листа (Тема)</label>
          <div class="grid grid-cols-3 gap-2">
            <button onclick={() => themeMode = 'paper'} class="h-10 border rounded-lg font-serif text-xs transition-all flex items-center justify-center gap-1.5 {themeMode === 'paper' ? 'border-black bg-black text-white' : 'border-black/10 bg-white text-black/70 hover:bg-gray-50'}">
              📄 Бумага
            </button>
            <button onclick={() => themeMode = 'sepia'} class="h-10 border rounded-lg font-serif text-xs transition-all flex items-center justify-center gap-1.5 {themeMode === 'sepia' ? 'border-amber-700 bg-amber-50 text-amber-900 font-bold' : 'border-black/10 bg-amber-52/20 text-amber-800 hover:bg-amber-50/50'}">
              📜 Сепия (Книжная)
            </button>
            <button onclick={() => themeMode = 'dark'} class="h-10 border rounded-lg font-serif text-xs transition-all flex items-center justify-center gap-1.5 {themeMode === 'dark' ? 'border-slate-800 bg-slate-900 text-slate-100' : 'border-black/10 bg-white text-black/70 hover:bg-gray-50'}">
              🌙 Ночной режим
            </button>
          </div>
        </div> -->

      </div>

      <!-- ФУТЕР ОКНА НАСТРОЕК -->
      <footer class="p-4 bg-gray-50 border-t border-black/5 flex justify-end gap-2 shrink-0">
        <button onclick={() => show = false} class="h-9 px-4 rounded-lg border border-black/10 bg-white font-mono text-md hover:bg-black hover:text-white transition-all active:scale-95">
          Отмена
        </button>
        <button onclick={saveSettings} class="h-9 px-5 rounded-lg bg-black text-white font-mono text-md border border-black/10 hover:bg-white hover:text-black transition-all active:scale-95">
          Применить изменения
        </button>
      </footer>
    </div>
  </div>
{/if}