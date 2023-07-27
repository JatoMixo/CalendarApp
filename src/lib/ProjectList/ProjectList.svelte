<script lang="ts">
    let projects: {name: string, color: string, description: string, start_date: {day: number, month: number, year: number}, final_date: {day: number, month: number, year: number}}[] = [
        {name: "Rickroll", color: "asd", description: "Rickroll or soemthing", start_date: {day: 6, month: 7, year: 2023}, final_date: {day: 7, month: 7, year: 2023}}
    ];
</script>

<style lang="scss">

    @import "/src/variables.scss";

    #main-box {
        background-color: $calendar-background;

        border: solid 4px $grey-color;
        border-radius: 15px;
        box-shadow: 0 0 10px $grey-color;

        width: 264px;

        display: flex;
        flex-direction: column;

        padding: 10px;
    }

    .text {
        margin: 0;

        text-align: left;
    }

    .project-name {

    }

    .project-description {

    }
</style>

<div id="main-box">
    {#each projects as project}
        <div class="project-container">
            <div class="project-text">
                <h1 class="project-name text">{project["name"]}</h1>
                <p class="project-description text">{project["description"]}</p>
            </div>
            
            <!--Icon for fucking deleting project-->
        </div>
    {/each}
</div>