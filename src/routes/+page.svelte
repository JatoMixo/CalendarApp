<script lang="ts">
    import Day from "$lib/Day.svelte";

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

<h1>Testing...</h1>
<p>Just... Stay here for a little bit more, I swear this is gonna look good at some point</p>

<div id="calendar-grid">
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
    <div class="day-element">
        <p>Just testing</p>
    </div>
</div>

<Day projects={example_projects} day_number="12"/>