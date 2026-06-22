<script lang="ts">
  import type { AssetPanelManager } from "$lib/scenario/assets.svelte";

  // Принимаем менеджер активов как проп Svelte 5
  let { assets }: { assets: AssetPanelManager } = $props();
</script>

{#if assets.showTooltip && assets.hoveredChar}
  <!-- 
    ИСПРАВЛЕНО: Ширина увеличена до w-[420px] для вместительности.
    Добавлен p-6 для больших внутренних отступов.
  -->
  <div 
    class="fixed z-9999 w-[520x] bg-white/95 border border-black/10 rounded-2xl p-5 shadow-2xl backdrop-blur-md pointer-events-none font-sans transition-all duration-200 animate-in fade-in zoom-in-95"
    style="left: {assets.tooltipPos.x}px; top: {assets.tooltipPos.y}px;"
  >
    <!-- Шапка досье: Увеличен размер аватарки (w-12 h-12) и текста заголовков -->
    <div class="flex items-center gap-4 border-b border-black/5 pb-4 mb-4">
      <div class="w-12 h-12 bg-black/5 rounded-full flex items-center justify-center text-2xl opacity-70 shrink-0">
        👤
      </div>
      <div class="min-w-0">
        <!-- text-base вместо text-sm для ФИО -->
        <h5 class="font-serif text-base font-bold leading-tight capitalize text-black/90 truncate">
          {assets.hoveredChar.first_name || "Имя"} {assets.hoveredChar.last_name || ""}
        </h5>
        <!-- text-[11px] вместо text-[10px] для возраста -->
        <span class="font-mono text-[11px] uppercase opacity-40 tracking-widest block mt-1">
          {assets.hoveredChar.age ? `${assets.hoveredChar.age} лет` : "Возраст не указан"}
        </span>
      </div>
    </div>

    <!-- Параметры анкеты: увеличен размер шрифта до text-[13px] и межстрочный интервал leading-relaxed -->
    <div class="space-y-4 text-[15px] leading-relaxed text-black/80 max-h-80 overflow-y-auto pr-1 scrollbar-thin">
      {#if assets.hoveredChar.habits}
        <p>
          <span class="font-mono text-[15px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Привычки:</span> 
          <span class="text-[14px]">{assets.hoveredChar.habits}</span>
        </p>
      {/if}
      
      {#if assets.hoveredChar.likes}
        <p>
          <span class="font-mono text-[15px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Любит:</span> 
          <span class="text-[14px]">{assets.hoveredChar.likes}</span>
        </p>
      {/if}
      
      {#if assets.hoveredChar.dislikes}
        <p>
          <span class="font-mono text-[15px] opacity-40 uppercase font-bold block mb-1 tracking-wider">Не любит:</span> 
          <span class="text-[14px]">{assets.hoveredChar.dislikes}</span>
        </p>
      {/if}

      {#if assets.hoveredChar.description}
        <!-- Крупный, комфортный для чтения блок биографии -->
        <p class="italic opacity-95 pt-3 border-t border-black/5 mt-3">
          <span class="font-mono text-[15px] not-italic opacity-40 uppercase font-bold block mb-1 tracking-wider">Биография / Психотип:</span> 
          <span class="line-clamp-6 block not-italic text-black/70 font-sans text-[14px] leading-relaxed">{assets.hoveredChar.description}</span>
        </p>
      {/if}
    </div>
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
