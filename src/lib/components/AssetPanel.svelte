<script lang="ts">
  import type { AssetPanelManager } from "$lib/scenario/assets.svelte";

  // Принимаем экземпляр менеджера активов как проп Svelte 5
  let { assets }: { assets: AssetPanelManager } = $props();
</script>

<!-- НИЖНЯЯ ПАНЕЛЬ РЕСУРСОВ (ЗЕРКАЛЬНОЕ ОТРАЖЕНИЕ EDITPANEL) -->
<div class="fixed inset-x-0 bottom-0 z-50 h-56 pointer-events-none overflow-hidden select-none">
  <div
    class="absolute bottom-0 left-1/2 flex flex-col h-48 w-[600px] -translate-x-1/2 translate-y-[calc(100%-12px)] 
           rounded-t-2xl border border-b-0 border-black/10 bg-white/80 shadow-2xl backdrop-blur-xl 
           transition-transform duration-300 ease-out hover:translate-y-0 pointer-events-auto"
  >
    <!-- Горизонтальная ручка триггера (Точно такие же 12px и стиль, как у твоей правой панели) -->
    <div
      class="flex h-3 w-full items-center justify-center cursor-pointer border-b border-black/5 bg-gray-50/50 rounded-t-2xl shrink-0"
    >
      <span class="font-mono text-[9px] text-black/30 uppercase tracking-widest whitespace-nowrap leading-none mb-0.5">
        ///
      </span>
    </div>

    <!-- Контент внутри шторки (Точно такие же gap, отступы p-5 и шрифты) -->
    <div class="flex flex-1 flex-col gap-4 p-5 font-sans overflow-hidden">
      
      <!-- Заголовок и вкладки на одном уровне -->
      <header class="flex items-center justify-between border-b border-black/5 pb-1 shrink-0">
        <h3 class="font-mono text-[12px] uppercase tracking-widest text-black/40 font-bold">
          Ресурсы проекта
        </h3>
        
        <!-- Вкладки, оптимизированные под уникальные атрибуты Svelte 5 -->
        <div class="flex gap-2">
          <button
            onclick={() => assets.toggleFolder("characters")}
            class="font-mono text-[11px] uppercase tracking-wider px-2 py-0.5 rounded transition-all outline-none 
                   {assets.activeFolder === 'characters' ? 'text-black font-bold' : 'text-black/40 font-normal'}"
          >
            Персонажи
          </button>
          
          <span class="text-black/20 font-mono text-[11px] select-none">|</span>
          
          <button
            onclick={() => assets.toggleFolder("locations")}
            class="font-mono text-[11px] uppercase tracking-wider px-2 py-0.5 rounded transition-all outline-none 
                   {assets.activeFolder === 'locations' ? 'text-black font-bold' : 'text-black/40 font-normal'}"
          >
            Локации
          </button>
        </div>
      </header>

      <!-- Контейнер вывода карточек активов с горизонтальным скроллом -->
      <div class="flex-1 overflow-x-auto flex gap-3 items-center pb-2 scrollbar-thin">
        {#if assets.activeFolder === 'characters'}
          {#each assets.characterList as file}
            <!-- 
              Карточка актива под стать кнопкам макросов (h-11, rounded-lg, border).
              Вместо ненадежного нативного draggable используем onmousedown для виртуального переноса.
            -->
            <div
              onmousedown={(e) => assets.startVirtualDrag(e, file)}
              class="h-11 px-3 shrink-0 rounded-lg border border-black/10 bg-white text-black/70 font-mono text-[13px] 
                     transition-all hover:bg-black hover:text-white hover:border-black active:scale-[0.98] 
                     cursor-grab active:cursor-grabbing flex items-center justify-between min-w-[140px] shadow-sm select-none"
            >
              <span class="capitalize truncate pr-2">{file.replace(".writer", "").replace(/_/g, " ")}</span>
              <span class="text-[10px] opacity-30 select-none shrink-0">👤</span>
            </div>
          {:else}
            <p class="font-mono text-xs opacity-30 w-full text-center py-4">Папка персонажей пуста</p>
          {/each}
        {:else if assets.activeFolder === 'locations'}
          <!-- Папка Локаций -->
          <div
            class="h-11 px-3 shrink-0 rounded-lg border border-dashed border-black/10 text-black/40 font-mono text-[12px] 
                   flex items-center justify-center w-full"
          >
            Папка локаций пуста (В разработке)
          </div>
        {:else}
          <!-- Начальное состояние: подсказка автору -->
          <div class="w-full text-center font-mono text-xs text-black/30 uppercase tracking-wide py-4">
            ← Выберите вкладку выше, чтобы открыть папку
          </div>
        {/if}
      </div>

    </div>
  </div>
</div>

<!-- 🌟 ЛЕТАЮЩИЙ ЗА МЫШКОЙ «ПРИЗРАК» КАРТОЧКИ (Отображается только во время переноса) -->
{#if assets.isDragging}
  <div 
    class="fixed z-[9999] px-4 h-11 border border-black bg-black text-white rounded-lg font-mono text-[13px] 
           pointer-events-none flex items-center justify-center shadow-2xl opacity-90 animate-in fade-in duration-150"
    style="left: {assets.dragPos.x + 12}px; top: {assets.dragPos.y + 12}px;"
  >
    <span class="mr-2">👤</span>
    <span class="font-bold tracking-wide">{assets.draggedText}</span>
  </div>
{/if}
<style>
  /* Тонкий кастомный скроллбар для аккуратного вывода карточек */
  .scrollbar-thin::-webkit-scrollbar {
    height: 4px;
  }
  .scrollbar-thin::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.08);
    border-radius: 4px;
  }
  .scrollbar-thin::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.15);
  }
</style>
