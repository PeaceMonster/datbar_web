
export type EventDetails = {
    startDate: Date,
    endDate: Date,
    answerBy: Date,
    title: string,
    location: string,
    description: string,
}



export function get_string_month(month: number) : string {
    switch (month) {
        case 0: 
            return "januar";
        case 1:
            return "febuar";
        case 2:
            return "marts";
        case 3:
            return "april";
        case 4:
            return "maj";
        case 5:
            return "juni";
        case 6:
            return "juli";
        case 7:
            return "august";
        case 8:
            return "september";
        case 9:
            return "oktober";
        case 10:
            return "november";
        case 11:
            return "december";
        default:
            return "Someone done goofed";
    }
}

export function get_date_string_time(date : Date ) : string {
    return date.getDate() + ". " + get_string_month(date.getMonth()) + " " + date.getFullYear() + " " + date.getHours() + ":" + date.getMinutes();
}

export function get_date_string(date : Date) : string {
    return date.getDate() + ". " + get_string_month(date.getMonth()) + " " + date.getFullYear();
}