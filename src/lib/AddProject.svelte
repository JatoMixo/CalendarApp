<script lang="ts">
    let name = "";
    let description = "";

    let color = "#000000";

    let start_date_string: string = "";
    let final_date_string: string = "";

    $: start_date = {
        day: +start_date_string.split("-")[2],
        month: +start_date_string.split("-")[1] - 1,
        year: +start_date_string.split("-")[0],
    };

    $: final_date = {
        day: +final_date_string.split("-")[2],
        month: +final_date_string.split("-")[1] - 1,
        year: +final_date_string.split("-")[0],
    };

    $: get_final_date_limit = () => {
        if (start_date_string == "") {
            return new Date().toISOString().slice(0, 10);
        }

        return start_date_string;
    }

    import { invoke } from "@tauri-apps/api";

    async function AddProject() {
        await invoke("add_project_to_cache_from_ui", {project: {name: name, color: color, description: description, start_date: start_date, final_date: final_date}});

        name = "";
        description = "";

        color = "#000000";

        start_date_string = "";
        final_date_string = "";
    }
</script>

<style lang="scss">

    @import "../variables.scss";

    #main-box {
        background-color: $calendar-background;

        border: solid 4px $grey-color;
        border-radius: 15px;
        box-shadow: 0 0 10px $grey-color;

        width: 264px;
        height: fit-content;

        display: flex;
        flex-direction: column;
        justify-content: center;

        padding: 10px;
    }

    $input-height: 25px;

    input {
        height: $input-height;

        margin-bottom: 10px;

        border: solid 4px $grey-color;
        border-radius: 10px;
        box-shadow: 0 0 5px $grey-color;

        outline: none;

        background-color: $dark-color;

        color: white;
        text-shadow: 0 0 2px white;

        font-family: "Mononoki NF";

        text-align: center;
    }

    #description-input {
        width: 210px;
    }

    ::placeholder {
        color: $grey-color;
        text-shadow: 0 0 2px $grey-color;
        font-family: "Mononoki NF";

        text-align: center;
    }

    ::-webkit-calendar-picker-indicator {
        background-color: white;

        border-radius: 5px;

        margin-right: 5px;
    }

    ::-webkit-calendar-picker-indicator:hover {
        background-color: $grey-color;
    }

    ::-webkit-calendar-picker-indicator:active {
        background-color: grey;
    }

    #description-input {
        float: right;
    }

    #color-selector {
        float: left;

        height: 35px;
        width: 35px;

        padding: 0;

        margin: 0;
    }

    #add-project-button {
        height: 32px;

        border: solid 4px $grey-color;
        border-radius: 10px;
        box-shadow: 0 0 5px $grey-color;

        background-color: #004b15;
        transition: background-color 0.25s;

        color: white;
        text-shadow: 0 0 2px white;

        font-family: "Mononoki NF";
    }

    #add-project-button:hover {
        background-color: #00621B;
    }

    #add-project-button:active {
        background-color: #009128;
    }
</style>

<div id="main-box">
    <input type="text" placeholder="Name" bind:value={name} id="name-input"/>

    <div id="second-row">
        <input type="color" bind:value={color} id="color-selector"/>
        <input type="text" placeholder="Description" bind:value={description} id="description-input" />
    </div>

    <div id="date-section">
        <input type="date" placeholder="Start Date" bind:value={start_date_string} id="start-date-input" min={new Date().toISOString().slice(0, 10)} max={final_date_string}/>
        <input type="date" placeholder="Final Date" bind:value={final_date_string} id="final-date-input" min={get_final_date_limit()}/>
    </div>

    <button id="add-project-button" on:click={AddProject}>Add Project</button>
</div>