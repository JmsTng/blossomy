<script>
    let { format = $bindable() } = $props();
</script>

<form>
    <h3>settings!</h3>

    <label for="hourFormat">hour format</label>
    <select name="hourFormat" id="hourFormat" bind:value={format}>
        <option value="h11">00:00 AM</option>
        <option value="h12">12:00 AM</option>
        <option value="h23">00:00</option>
        <option value="h24">24:00</option>
    </select>
</form>