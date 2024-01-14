<script lang="ts">
    let udlejning = [];
</script>

<h2>Reservation af fadølsanlæg</h2>
<h4>Fadølsanlæggene er allerede reserveret følgende dage</h4>
<ul>
    {#if udlejning.length == 0}
        <li>Anlæggene er pt ikke reserveret på noget tidspunkt</li>
    {/if}
</ul>
<h3>Regler for udlån</h3>
<p>
    Hvis vores anlæg ikke allerede er udlånt, kan du låne det ved at
    udfyldeansøgninsformularen nederst på denne side. Anlæg udlånes kun til 
    <b>interne arrangementer</b>
    eller til <b>arrangementer med tilknytning til baren</b>.
</p>
<p>
    <b>Interne arrangementer</b>: Udelukkende arrangementer der finder sted på
    universitetet. Dette inkluderer:
</p>
<ul>
    <li>Instittutets arrangementer</li>
    <li>Specialereceptioner</li>
</ul>
<p>
    <b>Arrangementer med tilknytning</b>: Arrangementer der finder sted på
    universitetet eller andetsteds, kan være af privat natur. Dette inkluderer:
</p>
<ul>
    <li>Eksterne virksomheder, der af en årsag har tilknytning til baren</li>
    <li>Arrangementer af bestyrelsesmedlemmer</li>
    <li>I nogle tilfælde, arrangementer af bartendere</li>
</ul>

<p>Indtil videre dækker følgende regler for de to grupper:</p>
<p>
    <b>Interne arrangermenter</b>: Forløber som det har gjort indtil nu, udlån
    inkluderer opstilling, kulsyre og skylning af anlægget, 50 glas (1 rulle)
    tilkøbes for 150,- pr. stk.
</p>
<p>
    <b>Arrangementer med tilknytning</b>: Her inkluderer udlån udelukkende oplås
    og aflås af fadølsanlægget. Det vil sige at anlægget skal afhentes i Ada-0,
    og afleveres rengjort i Ada-0. Afleveringstid af anlægget skal aftales i
    dialog med den ansvarlige for det praktiske, og er ikke nødvendigvis samme
    dag.
</p>
<h4>Anmod om udlejning i god tid</h4>
<p>
    Når du låner et anlæg, skal der på udlåningsdagen samt tilbagelevering, være
    et bestyrelsesmedlem der står for det praktiske. Vi er alle frivillige og
    har studier og jobs ved siden af vorest bestyrelsesarbejde, og derfor kan vi
    ikke være sikre på at kunne stille et besyterelsesmedlem til rådighed.
    <br />
    Derfor skal du, for at være sikker på at kunne låne anlægget,
    <b>skrive mindst 7 dage i forvejen</b>. Vi vil derfor, senest 5 dage før,
    melde ud hvorvidt nogen i bestyrelsen har mulighed for at være der i en af
    de mulige tidsrum. Hvis ikke du skriver mindst 7 dage i forvejen, kan vi
    ikke garantere at vi kan melde ud hvorvidt udlejning er muligt i god tid,
    dog er det stadig muligt at kunne låne anlægget selv hvis man melder ud i
    dårlig tid, vi kan blot ikke garantere at der kommer et svar i god tid.
</p>
<h3>
    <span class="mw-headline" id="Generelle_informationer">
        Generelle informationer
    </span>
</h3>
<ul>
    <li>Der betales per åbnet fustage.</li>
    <li>
        Vores primære fadøl er
        <a
            class="external text"
            href="http://aarhusbryghus.dk/homepage/bestil-aarhus-fadoel/extra-pilsner/"
            rel="nofollow"
        >
            Aarhus Ekstra Pilsner
        </a>
        , som kommer i 25 liters fustager. Prisen er 750 kr. pr. fustage (incl. moms)
    </li>
    <li>
        Udover den primære fadøl, kan man få de øl som står inde på Aarhus
        Bryghus' hjemmeside, som fast kan fås på fad. Dvs.
        <a
            class="external text"
            href="http://aarhusbryghus.dk/homepage/bestil-aarhus-fadoel/klosterbryg/"
            rel="nofollow">Klosterbryg</a
        >,
        <a
            class="external text"
            href="http://aarhusbryghus.dk/homepage/bestil-aarhus-fadoel/foraarsbryg/"
            rel="nofollow">Forårsbryg</a
        >,
        <a
            class="external text"
            href="http://aarhusbryghus.dk/homepage/bestil-aarhus-fadoel/celebration/"
            rel="nofollow">Celebration</a
        >
        og
        <a
            class="external text"
            href="http://aarhusbryghus.dk/homepage/bestil-aarhus-fadoel/blondie/"
            rel="nofollow">Blondie</a
        >
        (afhængig af sæson). Prisen er 800 kr. pr. fustage (incl. moms)
    </li>
    <li>
        Vi sælger også Magners Cider på fustage, der koster 1250 kr. pr. fustage
        (incl. moms).
    </li>
    <li>
        Vi har et sortiment af special-øl tilgængeligt på: (det er ikke
        tilgængelig pt. men man kan skrive en mail til <a
            class="external text"
            href="mailto:best@fredagscafeen.dk"
            rel="nofollow">best@fredagscafeen.dk</a
        >)
    </li>
    <li>
        Derudover leverer vi også sodavand og chips. Sodavand koster 5 kr/stk og
        chips koster 20kr/pose, der betales pr. åbnet enhed.
    </li>
</ul>
<h3>
    <span class="mw-headline" id="rentingform"> Ansøgninsformular </span>
</h3>

<form method="post" action="/udlejning/#rentingform">
	<div class="form-group">
        <label class="control-label" for="id_dateFrom">Start dato & tid</label><div class="input-group date">
    <input type="text" name="dateFrom" class="form-control" placeholder="Start dato & tid" title="Hvornår skal anlægget afhentes/stilles op?" required id="id_dateFrom">
    <div class="input-group-addon input-group-append" data-target="#datetimepicker1" data-toggle="datetimepickerv">
        <div class="input-group-text"><i class="glyphicon glyphicon-calendar"></i></div>
    </div>
</div>

    <div class="help-block">Hvornår skal anlægget afhentes/stilles op?</div>

</div>
<div class="form-group"><label class="control-label" for="id_dateTo">Slut dato & tid</label><div class="input-group date">
    <input type="text" name="dateTo" class="form-control" placeholder="Slut dato & tid" title="Hvornår skal anlægget afleveres/pilles ned?" required id="id_dateTo">
    <div class="input-group-addon input-group-append" data-target="#datetimepicker1" data-toggle="datetimepickerv">
        <div class="input-group-text"><i class="glyphicon glyphicon-calendar"></i></div>
    </div>
</div>

    <div class="help-block">Hvornår skal anlægget afleveres/pilles ned?</div>

</div>
<div class="form-group"><label class="control-label" for="id_whoReserved">Hvem er I?</label><input type="text" name="whoReserved" maxlength="140" class="form-control" placeholder="Hvem er I?" title="" required id="id_whoReserved"></div>
<div class="form-group"><label class="control-label" for="id_contactEmail">Email til kontaktperson</label><input type="email" name="contactEmail" maxlength="254" class="form-control" placeholder="Email til kontaktperson" title="" required id="id_contactEmail"></div>
<div class="form-group"><label class="control-label" for="id_contactPhone">Telefonnummer til kontaktperson</label><input type="number" name="contactPhone" class="form-control" placeholder="Telefonnummer til kontaktperson" title="" required id="id_contactPhone"></div>
<div class="form-group"><label class="control-label" for="id_whoPays">Hvem betaler?</label><input type="text" name="whoPays" maxlength="140" class="form-control" placeholder="Hvem betaler?" title="Hvem skal regningen sendes til? (Fulde navn på person, virksomhed eller organisation)" required id="id_whoPays">

    <div class="help-block">Hvem skal regningen sendes til? (Fulde navn på person, virksomhed eller organisation)</div>

</div>
<div class="form-group"><label class="control-label" for="id_paymentType">Betalingsform</label><select name="paymentType" class="form-control" title="Hvordan bliver der betalt?" required id="id_paymentType">
  <option value="" selected>---------</option>

  <option value="EAN">EAN</option>

  <option value="invoice">Faktura</option>

  <option value="card">Kort i baren</option>

</select>

    <div class="help-block">Hvordan bliver der betalt?</div>

</div>
<div class="form-group"><label class="control-label" for="id_EANnumber">EAN-nummer</label><input type="number" name="EANnumber" min="-9223372036854775808" max="9223372036854775807" class="form-control" placeholder="EAN-nummer" title="Skal kun angives, hvis der skal faktureres til et EAN-nummer" id="id_EANnumber">

    <div class="help-block">Skal kun angives, hvis der skal faktureres til et EAN-nummer</div>

</div>
<div class="form-group"><label class="control-label" for="id_where">Lokation</label><textarea name="where" cols="40" rows="10" maxlength="140" class="form-control" placeholder="Lokation" title="Hvor bliver arrangmentet afholdt?" required id="id_where"></textarea>

    <div class="help-block">Hvor bliver arrangmentet afholdt?</div>

</div>
<div class="form-group"><label class="control-label" for="id_expectedConsummation">Forventet forbrug</label><textarea name="expectedConsummation" cols="40" rows="10" maxlength="140" class="form-control" placeholder="Forventet forbrug" title="Hvilke slags øl eller andre drikkevarer ønskes der og hvor mange fustager af hver type?" required id="id_expectedConsummation"></textarea>

    <div class="help-block">Hvilke slags øl eller andre drikkevarer ønskes der og hvor mange fustager af hver type?</div>

</div>
<div class="form-group"><label class="control-label" for="id_comments">Kommentarer</label><textarea name="comments" cols="40" rows="10" class="form-control" placeholder="Kommentarer" title="" id="id_comments"></textarea></div>

<div class="form-group">
    <button class="btn btn-primary" type="submit"><span class="glyphicon glyphicon-send"></span> Indsend</button>
</div>

</form>

<style>
</style>
