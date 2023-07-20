<script lang="ts">
    export let project = {name: "", description: "", color: "", start_date: {day: 0, month: 0, year: 0}, final_date: {day: 0, month: 0, year: 0}};
    export let day_number = "";
</script>

<style lang="scss">

    $grey-color: #b7bac1;

    #main-box {
        background-color: #1c1c1c;

        width: 115px;
        height: 150px;

        border: solid 4px $grey-color;
        border-radius: 15px;
        box-shadow: 0 0 5px $grey-color;

        padding: 0;
    }

    #day-number {
        color: white;
        text-shadow: 0 0 5px $grey-color;

        font-size: 20px;

        margin-top: 6px;
        margin-left: 6px;
        margin-bottom: 0;
    }

    #project {

        margin-top: 0;

        border-radius: 15px;

        padding: 5px;

        h1 {
            margin: 0;
            font-size: 25px;
        }

        p {
            margin: 0;
            font-size: 15px;
        }

        height: 110px;
    }
</style>

<div id="main-box">
    <h1 id="day-number">{day_number}</h1>

    <div id="project" style="background-color: {project.color};">
        <h1>{project.name}</h1>
        <p>{project.description}</p>
    </div>
</div>