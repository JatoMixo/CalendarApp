<script lang="ts">
    let name = "";
    let description = "";

    let start_date: Date;
    let final_date: Date;

    function AddProject() {

    }
</script>

<style lang="scss">

    @import "../variables.scss";

    #main-box {
        background-color: $calendar-background;

        border: solid 4px $grey-color;
        border-radius: 15px;
        box-shadow: 0 0 10px $grey-color;

        width: fit-content;
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
    <input type="text" placeholder="Description" bind:value={description} id="description-input" />

    <div id="date-section">
        <input type="date" placeholder="Start Date" bind:value={start_date} id="start-date-input" />
        <input type="date" placeholder="Final Date" bind:value={final_date} id="final-date-input" />
    </div>

    <button id="add-project-button" on:click={AddProject}>Add Project</button>
</div>