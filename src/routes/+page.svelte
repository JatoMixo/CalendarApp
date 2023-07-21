<script lang="ts">
    import Day from "$lib/Day.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    /* ====== TO DELETE ====== */
    const example_projects = [{
        
        name: "Test",
        color: "#000000",
        description: "A small test",
        start_date: {
            day: 10,
            month: 3,
            year: 2023
        },
        final_date: {
            day: 11,
            month: 3,
            year: 2023,
        },
    },
        {name: "Test",
        color: "#000000",
        description: "A small test",
        start_date: {
            day: 10,
            month: 3,
            year: 2023
        },
        final_date: {
            day: 11,
            month: 3,
            year: 2023,
        },}
    ];
</script>

<style lang="scss">
</style>

<h1>Calendar App</h1>

<div id="days-row">
    <p>Monday</p>
    <p>Tuesday</p>
    <p>Wednesday</p>
    <p>Thursday</p>
    <p>Friday</p>
    <p>Saturday</p>
    <p>Sunday</p>
</div>

<div id="calendar-grid">
    {#each [...Array(31).keys()] as day}
        <Day day_number={day.toString()} project={example_projects[0]}/>
    {/each}
</div>