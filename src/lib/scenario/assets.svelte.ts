import { invoke } from "@tauri-apps/api/core";

export class AssetPanelManager {
  activeFolder = $state<"none" | "characters" | "locations">("none");
  characterList = $state<string[]>([]);
  locationList = $state<string[]>([]);

  isDragging = $state(false);
  draggedText = $state("");
  dragPos = $state({ x: 0, y: 0 });
  dragType = $state<"character" | "location">("character");

  hoveredChar = $state<any>(null);
  tooltipPos = $state({ x: 0, y: 0 });
  showTooltip = $state(false);

  constructor() {}

  toggleFolder(folder: "characters" | "locations") {
    if (this.activeFolder === folder) {
      this.activeFolder = "none";
    } else {
      this.activeFolder = folder;
      if (folder === "characters") {
        this.loadCharacters();
      } else if (folder === "locations") {
        this.loadLocations();
      }
    }
  }

  async loadCharacters() {
    try {
      this.characterList = await invoke<string[]>("get_characters");
    } catch (e) {
      console.error("Не удалось загрузить персонажей:", e);
    }
  }

  async loadLocations() {
    try {
      this.locationList = await invoke<string[]>("get_locations");
    } catch (e) {
      console.error("Не удалось загрузить локации:", e);
    }
  }

  async startVirtualDrag(
    event: MouseEvent,
    fileName: string,
    type: "character" | "location",
  ) {
    event.preventDefault();
    this.dragType = type;

    const fileSlug = fileName.replace(".writer", "");

    if (type === "character") {
      const cleanName = fileSlug.replace(/_/g, " ").toUpperCase();
      this.draggedText = cleanName;
      (this as any)._currentDragSlug = fileSlug;
      this.initDragSequence(event);
    } else {
      (this as any)._currentDragSlug = fileSlug;

      try {
        const locData = await invoke<any>("read_location", {
          nameFile: fileName,
        });
        const typeScene = (locData.type_scene || "ИНТ.").toUpperCase();
        const locName = (
          locData.name || fileSlug.replace(/_/g, " ")
        ).toUpperCase();
        const timeDay = (locData.time_day || "ДЕНЬ").toUpperCase();

        this.draggedText = `${typeScene} ${locName} - ${timeDay}`;
        this.initDragSequence(event);
      } catch (e) {
        console.error(
          "Не удалось прочитать параметры локации для переноса:",
          e,
        );
      }
    }
  }

  private initDragSequence(event: MouseEvent) {
    this.isDragging = true;
    this.dragPos = { x: event.clientX, y: event.clientY };
    window.addEventListener("mousemove", this.moveVirtualDrag);
    window.addEventListener("mouseup", this.dropVirtualDrag);
  }

  moveVirtualDrag = (event: MouseEvent) => {
    this.dragPos = { x: event.clientX, y: event.clientY };
  };

  dropVirtualDrag = (event: MouseEvent) => {
    window.removeEventListener("mousemove", this.moveVirtualDrag);
    window.removeEventListener("mouseup", this.dropVirtualDrag);
    this.isDragging = false;

    const editorDiv = document.querySelector(
      '[role="textbox"]',
    ) as HTMLDivElement;
    if (!editorDiv) return;

    const rect = editorDiv.getBoundingClientRect();
    const mouseXRelative = event.clientX - rect.left;
    const mouseYRelative = event.clientY - rect.top;

    const correctedX = rect.left + mouseXRelative / 0.75;
    const correctedY = rect.top + mouseYRelative / 0.75;

    const elementAtPoint = document.elementFromPoint(correctedX, correctedY);

    if (editorDiv === elementAtPoint || editorDiv.contains(elementAtPoint)) {
      editorDiv.focus();

      let htmlToInsert = "";

      if (this.dragType === "character") {
        const slug = (this as any)._currentDragSlug || "";
        const leftMargin = "&nbsp;".repeat(35);
        htmlToInsert = `<div><br></div><div class="font-mono uppercase">${leftMargin}<span class="character-link cursor-help border-b border-dashed border-black/20 hover:border-black/60 transition-colors" data-char-slug="${slug}" contenteditable="false">${this.draggedText}</span></div><div><br></div>`;
      } else {
        const slug = (this as any)._currentDragSlug || "";
        htmlToInsert = `<div><br></div><div class="location-link cursor-help font-mono uppercase font-bold tracking-wide border-b border-dashed border-black/10 hover:border-black/40 transition-colors" data-loc-slug="${slug}" contenteditable="false">${this.draggedText}</div><div><br></div>`;
      }

      const selection = window.getSelection();
      if (selection) {
        let range: Range | null = null;
        if (document.caretRangeFromPoint) {
          range = document.caretRangeFromPoint(correctedX, correctedY);
        }

        if (range) {
          selection.removeAllRanges();
          selection.addRange(range);

          const template = document.createElement("template");
          template.innerHTML = htmlToInsert;
          range.insertNode(template.content);

          range.collapse(false);
        }
      }

      editorDiv.dispatchEvent(new Event("input", { bubbles: true }));
    }
  };

  handleMouseMove(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target) return;

    const charSpan = target.closest(".character-link");
    const locSpan = target.closest(".location-link");

    if (charSpan) {
      const slug = charSpan.getAttribute("data-char-slug");
      if (
        slug &&
        (!this.hoveredChar ||
          this.hoveredChar.slug !== slug ||
          !this.showTooltip ||
          this.hoveredChar.type !== "char")
      ) {
        invoke<any>("read_character_by_name", { name: slug })
          .then((data) => {
            this.hoveredChar = { ...data, slug, type: "char" };
            this.showTooltip = true;
          })
          .catch((err) => console.error(err));
      }
      this.calculateTooltipPosition(event);
    } else if (locSpan) {
      const slug = locSpan.getAttribute("data-loc-slug");
      if (
        slug &&
        (!this.hoveredChar ||
          this.hoveredChar.slug !== slug ||
          !this.showTooltip ||
          this.hoveredChar.type !== "loc")
      ) {
        invoke<any>("read_location_by_name", { name: slug })
          .then((data) => {
            this.hoveredChar = {
              name: data.name || slug.replace(/_/g, " "),
              type_scene: data.type_scene || "ИНТ.",
              time_day: data.time_day || "ДЕНЬ",
              lighting: data.lighting || "",
              interior_details: data.interior_details || "",
              description: data.description || "",
              slug,
              type: "loc",
            };
            this.showTooltip = true;
          })
          .catch((err) => console.error(err));
      }
      this.calculateTooltipPosition(event);
    } else {
      this.showTooltip = false;
      this.hoveredChar = null;
    }
  }

  private calculateTooltipPosition(event: MouseEvent) {
    let targetX = event.clientX + 20;
    let targetY = event.clientY - 240;

    if (targetY < 10) targetY = event.clientY + 20;
    if (targetX + 420 > window.innerWidth) targetX = event.clientX - 440;

    this.tooltipPos = { x: targetX, y: targetY };
  }
}

export function createAssetPanelManager() {
  return new AssetPanelManager();
}
