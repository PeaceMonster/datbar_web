<script lang='ts'>

    import { get_date_string, get_date_string_time, type EventDetails } from "./time";

    export let details : EventDetails = {
        startDate: new Date(Date.now()),
        endDate:  new Date(Date.now()),
        answerBy:  new Date(Date.now()),
        title: "Demo",
        location: "Ada",
        description: "Demo Event"
    };

</script>

<h3>{get_date_string(details.startDate)} - {details.title}</h3>
<p>
    <strong>
        {get_date_string_time(details.startDate)}
    </strong>
    til 
    <strong>
        {get_date_string_time(details.endDate)}
    </strong>
</p>
<p>
    <strong>Tilmeldingsfrist:</strong> {get_date_string_time(details.answerBy)}
</p>
<p>
    <strong>Lokation:</strong> {details.location}
</p>
<p>
    <strong>Beskrivelse:</strong>
</p>
<p>
    {details.description}
</p>