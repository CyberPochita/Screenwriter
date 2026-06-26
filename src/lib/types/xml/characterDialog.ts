import type { Character } from "../character";
import type { parentheticalBlock } from "./parenthetical";

export interface characterDialogBlock {
    type: "Dialog",
    character: Character,
    text: String,
    parenthetical: parentheticalBlock
}