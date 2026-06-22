<!-- src/lib/components/ConfirmDialog.svelte -->
<script lang="ts">
  import { fade, scale } from "svelte/transition";

  let { 
    show = false, 
    title = "Удаление",
    message = "", 
    onConfirm = () => {}, 
    onCancel = () => {} 
  }: { 
    show: boolean; 
    title?: string;
    message: string; 
    onConfirm: () => void; 
    onCancel: () => void; 
  } = $props();
</script>

{#if show}
  <!-- Задний затемняющий и размывающий слой (Backdrop) -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    transition:fade={{ duration: 200 }}
    onclick={onCancel}
    class="fixed inset-0 z- bg-black/10 backdrop-blur-sm flex items-center justify-center p-4 select-none"
  >
    <!-- Само окно диалога -->
    <div 
      transition:scale={{ start: 0.95, duration: 200 }}
      onclick={(e) => e.stopPropagation()} 
      class="w-full max-w-sm bg-white/90 border border-black/10 rounded-2xl shadow-2xl backdrop-blur-xl p-6 font-mono text-xs tracking-wider uppercase text-slate-800"
    >
      <!-- Шапка -->
      <div class="flex flex-col mb-4">
        <span class="font-bold opacity-40 text-[12px] tracking-widest text-red-600">Предупреждение</span>
        <h3 class="mt-1 text-sm font-bold text-black">{title}</h3>
      </div>

      <!-- Текст предупреждения -->
      <p class="text-slate-600 font-sans normal-case text-sm leading-relaxed mb-6">
        {message}
      </p>

      <!-- Кнопки управления (Действия) -->
      <div class="flex gap-2 justify-end">
        <button 
          onclick={onCancel}
          class="px-4 py-2 border border-black/10 bg-white hover:bg-black text-black hover:text-white font-semibold rounded-xl transition-all active:scale-[0.98] cursor-pointer"
        >
          Отмена
        </button>
        <button 
          onclick={onConfirm}
          class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white font-semibold rounded-xl shadow-md transition-all active:scale-[0.98] cursor-pointer"
        >
          Удалить
        </button>
      </div>
    </div>
  </div>
{/if}
