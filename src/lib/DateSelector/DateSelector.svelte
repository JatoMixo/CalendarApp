<script lang="ts">
    export let month: string;
    export let year: number = 2023;

    const MONTHS: string[] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    import LeftArrow from "./left-arrow.png";
    import RightArrow from "./right-arrow.png";

    function NextMonth () {
        let month_index = MONTHS.indexOf(month);

        const MAX_MONTH_INDEX = MONTHS.length - 1;
        if (month_index == MAX_MONTH_INDEX) {
            month = MONTHS[0];
            year++;

            return;
        }

        month = MONTHS[month_index + 1];
    }

    function LastMonth () {
        let month_index = MONTHS.indexOf(month);

        if (month_index == 0) {
            month = MONTHS[MONTHS.length - 1];
            year--;

            return;
        }

        month = MONTHS[month_index - 1];
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
    <button id="left-button" class="change-button" on:click={LastMonth}>
        <img src={LeftArrow} alt="Back" class="arrow"/>
    </button>

    <p id="date-display">{month} {year}</p>

    <button id="right-button" class="change-button" on:click={NextMonth}>
        <img src={RightArrow} alt="Next" class="arrow"/>
    </button>
</div>