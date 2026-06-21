export interface IAgentInfo {
    name: string;
    company: string;
    phone: string;
    email: string;
}

export interface IContactInfo {
    name: string;
    // Разрешаем null для полной совместимости с Rust Option
    address?: string | null;
    phone?: string | null;
    email?: string | null;
    agent?: IAgentInfo | null;
}

// Точный маппинг структуры enum из Rust с ассоциативными полями ({ source: String })
export type TAuthorship = 
    | 'original' 
    | { adaptation: { source: string } } 
    | { basedOn: { source: string } }; // Если в Rust стоит #[serde(rename_all = "camelCase")]

export interface ITitlePage {
    title: string;
    author: string;
    authorship: TAuthorship;
    contact: IContactInfo;
}
