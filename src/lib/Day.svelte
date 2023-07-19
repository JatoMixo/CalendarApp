<script lang="ts">
    export let projects = [{}];
    export let day_number = "";
</script>

<style lang="scss">

    $grey-color: #b7bac1;

    #main-box {
        background-color: #1c1c1c;

        width: 125px;
        height: 200px;

        border: solid 4px $grey-color;
        border-radius: 15px;
        box-shadow: 0 0 5px $grey-color;

        padding: 6px;
    }

    #day-number {
        color: white;
        text-shadow: 0 0 5px $grey-color;

        font-size: 40px;

        margin: 0;
    }
</style>

<div id="main-box">
    <h1 id="day-number">{day_number}</h1>

</div>