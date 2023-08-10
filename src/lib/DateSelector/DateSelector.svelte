<script lang="ts">
    export let month: number = 0;
    export let year: number = 2023;

    import { emit } from "@tauri-apps/api/event";

    const MONTHS: string[] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    import LeftArrow from "./left-arrow.png";
    import RightArrow from "./right-arrow.png";

    function nextMonth () {
        if (month == MONTHS.length - 1) {
            month = 0;
            year++;

            return;
        }

        emit("changed_month");

        month++;
    }

    function backMonth () {
        if (month == 0) {
            year--;
            month = MONTHS.length - 1;

            return;
        }

        emit("changed_month");

        month--;
    }
</script>

<style lang="scss">
    @import "../../variables.scss";
    #main-container {
        display: flex;

        justify-content: center;

        gap: 50px;

        margin: 0;
    }

    #date-display {
        font-size: 30px;

        width: 300px;
        text-align: center;
    }

    $arrow-length: 50px;

    .change-button {
        background-color: $background-color;
        border-radius: 50%;

        width: $arrow-length;
        height: $arrow-length;

        border: 0;

        padding: 0;

        margin-top: auto;
        margin-bottom: auto;

        box-shadow: 0 0 5px white;
    }

    .arrow {
        width: $arrow-length;
        height: $arrow-length;

        margin: 0;

        opacity: 65%;
    }

    .arrow:hover {
        opacity: 75%;
    }

    .arrow:active {
        opacity: 100%;
    }
</style>

<div id="main-container">
    <button id="left-button" class="change-button" on:click={backMonth}>
        <img src={LeftArrow} alt="Back" class="arrow"/>
    </button>

    <p id="date-display">{MONTHS[month]} {year}</p>

    <button id="right-button" class="change-button" on:click={nextMonth}>
        <img src={RightArrow} alt="Next" class="arrow"/>
    </button>
</div>