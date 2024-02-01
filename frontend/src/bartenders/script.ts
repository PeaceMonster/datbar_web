import type { Bartender } from "../lib/types";

export async function get_bartenders() : Promise<Bartender[][]> {

    let result = await fetch("/api/bartenders");
    let bartenderArray : Bartender[] = await result.json();

    let active = bartenderArray.filter((b,_) => b.active).sort((a, b) => a.name.localeCompare(b.name, "DA"));
    let inactive = bartenderArray.filter((b,_) => !b.active).sort((a, b) => a.name.localeCompare(b.name, "DA"));

    return [active, inactive];
}