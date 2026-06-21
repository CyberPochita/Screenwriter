export interface IPageFormatting {
    top_margin: number;
    left_margin: number;
    right_margin: number;
    contact_left_margin: number;
}

export interface IPageContent {
    id: number; // Внутренний ID для CodeMirror (не улетает в XML)
    formatting: IPageFormatting;
    text: string;
}

export interface IScenarioDocument {
    pages: { formatting: IPageFormatting; text: string }[];
}