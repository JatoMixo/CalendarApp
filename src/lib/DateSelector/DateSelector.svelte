<script lang="ts">
    export let month: string;
    export let year: string;

    import LeftArrow from "./left-arrow.png";
    import RightArrow from "./right-arrow.png";
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
    <button id="left-button" class="change-button">
        <img src={LeftArrow} alt="Back" class="arrow"/>
    </button>

    <p id="date-display">{month} {year}</p>

    <button id="right-button" class="change-button">
        <img src={RightArrow} alt="Next" class="arrow"/>
    </button>
</div>