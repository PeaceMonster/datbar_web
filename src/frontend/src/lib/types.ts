export type Page = {
    component: ConstructorOfATypedSvelteComponent,
    name: string,
    route: string,
    menu: boolean,
    subitems: Page[] | null
}
