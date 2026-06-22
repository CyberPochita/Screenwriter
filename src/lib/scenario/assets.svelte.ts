import { invoke } from "@tauri-apps/api/core";

export class AssetPanelManager {
  // Реактивные состояния папок
  activeFolder = $state<"none" | "characters" | "locations">("none");
  characterList = $state<string[]>([]);

  // Состояния виртуального Drag-and-Drop
  isDragging = $state(false);
  draggedText = $state("");
  dragPos = $state({ x: 0, y: 0 });

  // 🌟 ИСПРАВЛЕНО: Добавлены недостающие свойства для ховера подсказок
  hoveredChar = $state<any>(null);
  tooltipPos = $state({ x: 0, y: 0 });
  showTooltip = $state(false);

  constructor() {}

  // Переключение состояния папок шторки
  toggleFolder(folder: "characters" | "locations") {
    if (this.activeFolder === folder) {
      this.activeFolder = "none";
    } else {
      this.activeFolder = folder;
      if (folder === "characters") {
        this.loadCharacters();
      }
    }
  }

  // Загрузка персонажей из бэкенда на Rust
  async loadCharacters() {
    try {
      this.characterList = await invoke<string[]>("get_characters");
    } catch (e) {
      console.error("Не удалось загрузить персонажей для панели:", e);
    }
  }

  // 1. Зажимаем мышку на карточке персонажа (Виртуальный старт)
  startVirtualDrag(event: MouseEvent, fileName: string) {
    event.preventDefault();
    
    // Генерируем slug (имя файла без расширения, например: potter_harry)
    const charSlug = fileName.replace(".writer", "");
    // Красивое имя для печати на листе (ГАРРИ ПОТТЕР)
    const cleanName = charSlug.replace(/_/g, " ").toUpperCase();
    
    this.isDragging = true;
    this.draggedText = cleanName;
    this.dragPos = { x: event.clientX, y: event.clientY };

    // Сохраняем slug текущего перетаскиваемого персонажа, чтобы передать его при сбросе
    (this as any)._currentDragSlug = charSlug;

    window.addEventListener("mousemove", this.moveVirtualDrag);
    window.addEventListener("mouseup", this.dropVirtualDrag);
  }

  // 2. Двигаем мышку — копия карточки летит за курсором
  moveVirtualDrag = (event: MouseEvent) => {
    this.dragPos = { x: event.clientX, y: event.clientY };
  };

  // 3. Отпускаем мышку — точечно вставляем HTML токен в текст
  dropVirtualDrag = (event: MouseEvent) => {
    window.removeEventListener("mousemove", this.moveVirtualDrag);
    window.removeEventListener("mouseup", this.dropVirtualDrag);
    this.isDragging = false;

    const editorDiv = document.querySelector('[role="textbox"]') as HTMLDivElement;
    if (!editorDiv) return;

    // 🌟 МАТЕМАТИЧЕСКИЙ РАСЧЕТ ДЛЯ КОРРЕКЦИИ ZOOM-[0.75]
    // Находим реальные физические координаты самого текстового редактора на экране
    const rect = editorDiv.getBoundingClientRect();
    
    // Вычисляем, на сколько пикселей мышка смещена относительно верхнего левого угла редактора
    const mouseXRelative = event.clientX - rect.left;
    const mouseYRelative = event.clientY - rect.top;

    // Корректируем эти координаты, деля их на масштаб 0.75
    // Это возвращает нам "честные" координаты внутри документа, как если бы зума не было
    const correctedX = rect.left + (mouseXRelative / 0.75);
    const correctedY = rect.top + (mouseYRelative / 0.75);

    // Ищем элемент по скорректированным координатам
    const elementAtPoint = document.elementFromPoint(correctedX, correctedY);

    // Проверяем, попали ли мы внутрь редактора
    if (editorDiv === elementAtPoint || editorDiv.contains(elementAtPoint)) {
      editorDiv.focus();

      const slug = (this as any)._currentDragSlug || "";
      const leftMargin = "&nbsp;".repeat(32);

      // Формируем чистый HTML-блок
      const htmlToInsert = `<div><br></div><div class="font-mono uppercase">${leftMargin}<span class="character-link cursor-help border-b border-dashed border-black/20 hover:border-black/60 transition-colors" data-char-slug="${slug}" contenteditable="false">${this.draggedText}</span></div><div><br></div>`;

      const selection = window.getSelection();
      if (selection) {
        let range: Range | null = null;
        
        // Передаем браузеру идеально скорректированные координаты X и Y
        if (document.caretRangeFromPoint) {
          range = document.caretRangeFromPoint(correctedX, correctedY);
        }

        if (range) {
          selection.removeAllRanges();
          selection.addRange(range);
          
          // Вставляем HTML точно в место скорректированного курсора
          const template = document.createElement("template");
          template.innerHTML = htmlToInsert;
          range.insertNode(template.content);
          
          // Сдвигаем каретку ввода в конец вставленного блока
          range.collapse(false);
        }
      }

      // Триггерим input для обновления bind:value в Svelte
      editorDiv.dispatchEvent(new Event("input", { bubbles: true }));
    }
  };

  // 🌟 ИСПРАВЛЕНО: Добавлен глобальный слушатель ховера для карточки предпросмотра
    handleMouseMove(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const charSpan = target.closest(".character-link");
    
    if (charSpan) {
      const slug = charSpan.getAttribute("data-char-slug");
      
      if (slug && (!this.hoveredChar || this.hoveredChar.slug !== slug || !this.showTooltip)) {
        invoke<any>("read_character_by_name", { name: slug })
          .then((data) => {
            this.hoveredChar = { ...data, slug };
            this.showTooltip = true;
          })
          .catch((err) => {
            console.error("Ошибка загрузки превью персонажа:", err);
            this.showTooltip = false;
          });
      }
      
      // 🌟 ИСПРАВЛЕНО: Динамический расчет координат для защиты от скролла
      let targetX = event.clientX + 20;
      let targetY = event.clientY - 240; // Базовое положение: выше курсора на высоту карточки

      // Защита от улета за верхнюю границу экрана (потолок)
      // Если карточка вылезает вверх (координата меньше 10px от края)
      if (targetY < 10) {
        // Перекидываем её НАДОБНО снизу под курсор, чтобы она не создавала скролл
        targetY = event.clientY + 20; 
      }

      // Защита от улета за правую границу экрана
      // 420px — это новая ширина нашей крупной карточки
      if (targetX + 420 > window.innerWidth) {
        // Сдвигаем карточку левее курсора мыши
        targetX = event.clientX - 440;
      }

      // Записываем финальные, абсолютно безопасные координаты
      this.tooltipPos = { x: targetX, y: targetY };
    } else {
      this.showTooltip = false;
      this.hoveredChar = null;
    }
  }

}

export function createAssetPanelManager() {
  return new AssetPanelManager();
}
