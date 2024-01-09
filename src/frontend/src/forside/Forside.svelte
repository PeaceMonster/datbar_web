<script lang='ts'>

</script>

<p>Velkommen til Fredagscaféens webside, her kan du finde information om åbningstider, sortiment, udlejning af fadølsanlæg og grill mm.</p>


<h2>Bartendertilmelding</h2>
<div class="col-md-8">

    <p>Er du interesseret i at være med i et fedt fællesskab hvor vi hygger bag baren og serverer nogle kolde drikkevarer?</p>
    <p>Du vil som bartender få ca. 2 vagter pr. semester, hvor du er med til at sikre at alle får slukket tørsten.</p>
    <p>Udover dette inviteres du til vores årlige arrangementer for bartendere, såsom Sommerfesten og Julefrokosten. Derudover vil der løbende være fede begivenheder, såsom ølsmagning og lign.</p>
    <p>Det er en god idé at læse <a href="/guides">barvagt-guiden</a> før du tilmelder dig, så du ved hvad en typisk vagt i Fredagscaféen indebærer.</p>

    <form method="post" action="/">
        <label class="control-label" for="id_name">Fulde navn</label>
        <input type="text" name="name" maxlength="140" class="form-control" placeholder="Fulde navn" required id="id_name">
        
        <label class="control-label" for="id_username">Brugernavn</label>
        <input type="text" name="username" maxlength="140" class="form-control" placeholder="Brugernavn"  required id="id_username">
        
        <label class="control-label" for="id_email">Email</label>
        <input type="text" name="email" maxlength="255" class="form-control" placeholder="Email"  required id="id_email">

        <label class="control-label" for="id_studentNumber">Studienummer</label>
        <input type="number" name="studentNumber" class="form-control" placeholder="Studienummer"  required id="id_studentNumber">

        <label class="control-label" for="id_phoneNumber">Telefonnummer</label>
        <input type="number" name="phoneNumber" class="form-control" placeholder="Telefonnummer"  required id="id_phoneNumber">
        
        <label class="control-label" for="id_tshirt_size">T-shirt størrelse</label>
        <select name="tshirt_size" class="form-control"  required id="id_tshirt_size">
            <option value="" selected>---------</option>
            <option value="XS">XS</option>
            <option value="S">S</option>
            <option value="M">M</option>
            <option value="L">L</option>
            <option value="XL">XL</option>
            <option value="XXL">XXL</option>
            <option value="XXXL">XXXL</option>
        </select>

        <label class="control-label" for="id_study">Studie</label>
        <input type="text" name="study" maxlength="50" class="form-control" placeholder="Studie"  required id="id_study">
        
        <label class="control-label" for="id_study_year">Årgang</label>
        <input type="number" name="study_year" class="form-control" placeholder="Årgang"  required id="id_study_year">
        
        <label class="control-label" for="id_info">Info</label>
        <textarea name="info" cols="40" rows="10" class="form-control" placeholder="Info" title="Eventuelle ekstra info til bestyrelsen skrives her" id="id_info"></textarea>
        <div class="help-block">Eventuelle ekstra info til bestyrelsen skrives her</div>

        <button class="btn btn-primary" type="submit"><span class="glyphicon glyphicon-send"></span> Indsend</button>
        
    </form>
</div>