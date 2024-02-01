export type Page = {
    component: ConstructorOfATypedSvelteComponent, // Sæt denne til P404.svelte, hvis der skal routes til en ny side Ex. admin
    name: string,
    route: string,
    subitems: Subpage[]
}

export type Subpage = {
    component: ConstructorOfATypedSvelteComponent,
    name: string,
    route: string,
}

export type Bartender = {
    name: string,
    username: string
    active: boolean,
}

export type Barvagt = {
    date: Date,
    resposible: Bartender,
    bartenders: Bartender[]
}