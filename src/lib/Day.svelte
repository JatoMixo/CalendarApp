<script lang="ts">
    export let project: any = null;
    export let dayNumber = "";
</script>

<style lang="scss">

    @import "../variables.scss";

    #main-box {
        background-color: $calendar-background;

        width: 115px;
        height: fit-content;
        min-height: 119px;

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

        height: 80px;
    }
</style>

<div id="main-box">
    <h1 id="day-number">{dayNumber}</h1>

    {#if project != null}
        <div id="project" style="background-color: {project.color};">
            <h1>{project.name}</h1>
            <p>{project.description}</p>
        </div>
    {/if}
</div>