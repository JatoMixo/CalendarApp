<script lang="ts">

    import TrashLogo from "./trash-can.png";

    let projects: Project[] = [
        {name: "Rickroll", color: "#ffffff", description: "Rickroll or soemthing", start_date: {day: 6, month: 7, year: 2023}, final_date: {day: 7, month: 7, year: 2023}},
        {name: "SomeTest", color: "#ffff00", description: "A test", start_date: {day: 8, month: 7, year: 2023}, final_date: {day: 9, month: 7, year: 2023}}
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

        gap: 15px;
    }

    .text {
        margin: 0;

        text-align: left;
    }

    .project-container {
        display: flex;
    }

    .project-name {
        font-size: 25px;
    }

    .project-description {
        font-size: 15px;
    }

    .delete-button {
        margin: -5px;

        width: fit-content;

        padding: 0;

        background-color: $calendar-background;

        display: flex;
        flex-direction: column;
        justify-content: center;

        border: solid 4px $calendar-background;
        border-radius: 10px;
    }

    .delete-button:hover {
        $hover-color: #222222;

        background-color: $hover-color;
        border: solid 4px $hover-color;
    }

    .delete-button:active {
        $clicked-color: #2a2a2a;

        background-color: $clicked-color;
        border: solid 4px $clicked-color;
    }

    .trash-icon {
        width: 40px;
        height: 40px;
    }

    .project-text {
        width: 400px;
    }
</style>

<div id="main-box">
    {#each projects as project}
        <div class="project-container">
            <div class="project-text">
                <h1 class="project-name text" style="color: {project.color};">{project.name}</h1>
                <p class="project-description text">{project.description}</p>
            </div>
            
            <button class="delete-button">
                <img src={TrashLogo} alt="Delete" class="trash-icon"/>
            </button>
        </div>
    {/each}
</div>