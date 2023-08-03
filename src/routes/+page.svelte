<script lang="ts">
    import Day from "$lib/Day.svelte";
    import DateSelector from "$lib/DateSelector/DateSelector.svelte";
    import AddProject from "$lib/AddProject.svelte";
    import ProjectList from "$lib/ProjectList/ProjectList.svelte";

    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";

    let actual_month = new Date().getMonth();
    let actual_year = new Date().getFullYear();

    $: first_day_of_month = new Date(actual_year, actual_month, 1).getDay();

    $: get_offset_days = () => {
        if (first_day_of_month == 0) {
            return 6;
        }

        return first_day_of_month - 1;
    }
    
    $: offset_days = get_offset_days();

    $: get_number_of_days_from_month = () => {
        
        const FEBRUARY_INDEX = 1;

        if (actual_month == FEBRUARY_INDEX) {
            return 28 + (actual_year % 4 == 0 ? 1 : 0);
        }

        if (actual_month <= 6) {
            return 30 + (actual_month % 2 == 0 ? 1 : 0);
        }

        return 30 + (actual_month % 2 == 0 ? 0 : 1);
    }

    let projects_for_days: any[] = [];

    async function get_projects_for_month() {
        projects_for_days = await invoke("get_projects_vector", {month: actual_month, year: actual_year});
    }

    get_projects_for_month();

    listen("added_project", () => {
        get_projects_for_month();
    });

    listen("removed_project", () => {
        get_projects_for_month();
    });

    listen("changed_month", () => {
        console.log("AS")
        get_projects_for_month();
    });
</script>

<div id="calendar-section">

    <div id="column-left">
        <DateSelector bind:month={actual_month} bind:year={actual_year}/>

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
            {#if offset_days != 0}
                <div style="grid-column-start: 1; grid-column-end: {offset_days + 1}"></div>
            {/if}
            
            {#each [...Array(get_number_of_days_from_month()).keys()] as day}
                <Day day_number={(day + 1).toString()} project={projects_for_days[day]}/>
            {/each}
        </div>
    </div>
    
    <div id="column-right">
        <h1>Add Project</h1>
        <AddProject />

        <h1>Project List</h1>
        <ProjectList />
    </div>
</div>

