import type { Bartender } from "../lib/types"

export function get_short_month(month : number) : string {
    switch (month) {
        case 0: return "Jan"
        case 1: return "Feb"
        case 2: return "Mar"   
        case 3: return "Apr"
        case 4: return "Maj"
        case 5: return "Jun"
        case 6: return "Jul"
        case 7: return "Aug"
        case 8: return "Sep"
        case 9: return "Oct"
        case 10: return "Nov"
        case 12: return "Dec"
        default: return "You done goofed"
    }
}

export function expand_bartender_list(list : Bartender[]) : string {
    let result = "";
    for(let i = 0; i < list.length; i++) {
        result += list[i].name;
        if (i + 1 < list.length) {
            result += ", ";
        }
    }
    return result
}