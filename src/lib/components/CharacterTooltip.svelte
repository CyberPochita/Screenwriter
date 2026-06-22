<script lang="ts">
  import type { AssetPanelManager } from "$lib/scenario/assets.svelte";

  let { assets }: { assets: AssetPanelManager } = $props();
</script>

{#if assets.showTooltip && assets.hoveredChar}
  <div 
    class="fixed z-50 w-[420px] bg-white/95 border border-black/10 rounded-2xl p-6 shadow-2xl backdrop-blur-md pointer-events-none font-sans transition-all duration-200 animate-in fade-in zoom-in-95"
    style="left: {assets.tooltipPos.x}px; top: {assets.tooltipPos.y}px;"
  >
    
    <!-- 👤 ОТОБРАЖЕНИЕ КАРТОЧКИ ПЕРСОНАЖА -->
    {#if assets.hoveredChar.type === 'char'}
      <div class="flex items-center gap-4 border-b border-black/5 pb-4 mb-4">
        <div class="w-12 h-12 bg-black/5 rounded-full flex items-center justify-center text-2xl opacity-70 shrink-0">
          👤
        </div>
        <div class="min-w-0">
          <h5 class="font-serif text-base font-bold leading-tight capitalize text-black/90 truncate">
            {assets.hoveredChar.first_name || "Имя"} {assets.hoveredChar.last_name || ""}
          </h5>
          <span class="font-mono text-[11px] uppercase opacity-40 tracking-widest block mt-1">
            {assets.hoveredChar.age ? `${assets.hoveredChar.age} лет` : "Возраст не указан"}
          </span>
        </div>
      </div>

      <div class="space-y-4 text-[13px] leading-relaxed text-black/80 max-h-[320px] overflow-y-auto pr-1 scrollbar-thin">
        {#if assets.hoveredChar.habits}
          <p><span class="font-mono text-[10px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Привычки:</span> {assets.hoveredChar.habits}</p>
        {/if}
        {#if assets.hoveredChar.likes}
          <p><span class="font-mono text-[10px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Любит:</span> {assets.hoveredChar.likes}</p>
        {/if}
        {#if assets.hoveredChar.dislikes}
          <p><span class="font-mono text-[10px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Не любит:</span> {assets.hoveredChar.dislikes}</p>
        {/if}
        {#if assets.hoveredChar.description}
          <p class="italic opacity-95 pt-3 border-t border-black/5 mt-3">
            <span class="font-mono text-[10px] not-italic opacity-40 uppercase font-bold block mb-1 tracking-wider">Биография / Психотип:</span> 
            <span class="line-clamp-6 block not-italic text-black/70 font-sans text-[13px] leading-relaxed">{assets.hoveredChar.description}</span>
          </p>
        {/if}
      </div>

    <!-- 📍 ОТОБРАЖЕНИЕ КАРТОЧКИ ЛОКАЦИИ (Новый блок!) -->
    {:else}
      <div class="flex items-center gap-4 border-b border-black/5 pb-4 mb-4">
        <div class="w-12 h-12 bg-black/5 rounded-full flex items-center justify-center text-2xl opacity-70 shrink-0">
          📍
        </div>
        <div class="min-w-0 flex-1">
          <h5 class="font-serif text-base font-bold leading-tight uppercase text-black/90 truncate">
            {assets.hoveredChar.type_scene} {assets.hoveredChar.name}
          </h5>
          <span class="font-mono text-[11px] uppercase opacity-40 tracking-widest block mt-1">
            Режим времени: {assets.hoveredChar.time_day}
          </span>
        </div>
      </div>

      <div class="space-y-4 text-[13px] leading-relaxed text-black/80 max-h-[320px] overflow-y-auto pr-1 scrollbar-thin">
        {#if assets.hoveredChar.lighting}
          <p>
            <span class="font-mono text-[10px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Освещение сцены:</span> 
            {assets.hoveredChar.lighting}
          </p>
        {/if}
        {#if assets.hoveredChar.interior_details}
          <p>
            <span class="font-mono text-[10px] opacity-40 uppercase font-bold block mb-1 tracking-wider">В фокусе камеры (Реквизит):</span> 
            {assets.hoveredChar.interior_details}
          </p>
        {/if}
        {#if assets.hoveredChar.description}
          <p class="italic opacity-95 pt-3 border-t border-black/5 mt-3">
            <span class="font-mono text-[10px] not-italic opacity-40 uppercase font-bold block mb-1 tracking-wider">Атмосфера места / Референсы:</span> 
            <span class="line-clamp-6 block not-italic text-black/70 font-sans text-[13px] leading-relaxed">{assets.hoveredChar.description}</span>
          </p>
        {/if}
      </div>
    {/if}

  </div>
{/if}

<style>
  .scrollbar-thin::-webkit-scrollbar {
    width: 4px;
  }
  .scrollbar-thin::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.08);
    border-radius: 4px;
  }
</style>
