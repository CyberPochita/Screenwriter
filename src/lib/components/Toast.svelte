<!-- src/lib/components/Toast.svelte -->
<script lang="ts">
  import { fly } from "svelte/transition";

  let { 
    message = null, 
    isError = false 
  }: { 
    message: string | null; 
    isError?: boolean 
  } = $props();

  /**
   * Нативная функция-экшен для Svelte 5.
   * Физически перемещает DOM-элемент в конец тега <body>,
   * полностью изолируя его от любых overflow ограничений редактора!
   */
  // svelte-ignore non_reactive_update
  function portal(node: HTMLElement) {
    document.body.appendChild(node);
    return {
      destroy() {
        if (node.parentNode) {
          node.parentNode.removeChild(node);
        }
      }
    };
  }
</script>

{#if message}
  <!-- 
    ИСПРАВЛЕНО: Добавлено use:portal. 
    Теперь тост рендерится на самом верхнем системном слое приложения.
  -->
  <div 
    use:portal
    class="fixed top-22 right-6 z-[9999] w-80 h-auto pointer-events-none overflow-hidden flex items-start justify-end pb-10"
  >
    
    <div
      transition:fly={{ x: 200, duration: 400 }}
      class="pointer-events-auto flex items-start gap-3 px-5 py-3.5 
             rounded-xl border shadow-lg backdrop-blur-xl font-mono text-xs tracking-wider normal-case w-full
             {isError 
               ? 'bg-red-500/10 border-red-500/20 text-red-500 shadow-red-700/10' 
               : 'bg-white/5 border-white/10 text-black/75 shadow-emerald-700/10'}"
    >
      <!-- Графический маркер статуса -->
      {#if isError}
        <span class="text-sm mt-0.5">⚠️</span>
      {:else}
        <span class="text-emerald-500 text-sm mt-0.5">✓</span>
      {/if}

      <!-- Текстовый блок сообщения в вашем стиле -->
      <div class="flex flex-col flex-1 min-w-0">
        <span class="font-bold opacity-40 text-[12px] tracking-widest">Система</span>
        <span class="mt-0.5 break-words whitespace-pre-wrap leading-relaxed normal-case font-sans font-medium text-[13px]">
          {message}
        </span>
      </div>
    </div>

  </div>
{/if}
