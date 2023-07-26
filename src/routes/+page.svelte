<script lang="ts">
    import Day from "$lib/Day.svelte";
    import DateSelector from "$lib/DateSelector/DateSelector.svelte";
    import AddProject from "$lib/AddProject.svelte";
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

<div id="calendar-section">

    <div id="column-left">
        <DateSelector month="July"/>

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
                <Day day_number={(day + 1).toString()} project={example_projects[0]}/>
            {/each}
        </div>
    </div>
    
    <div id="column-right">
        <h1>Add Project</h1>
        <AddProject />

        <h1>Project List</h1>
        <!-- === PROJECT LIST INCOMING === -->
    </div>
</div>

