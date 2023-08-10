<script lang="ts">
    import Day from "$lib/Day.svelte";
    import DateSelector from "$lib/DateSelector/DateSelector.svelte";
    import AddProject from "$lib/AddProject.svelte";
    import ProjectList from "$lib/ProjectList/ProjectList.svelte";

    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";

    let actualMonth = new Date().getMonth();
    let actualYear = new Date().getFullYear();

    $: firstDayOfMonth = new Date(actualYear, actualMonth, 1).getDay();

    $: getOffsetDays = () => {
        if (firstDayOfMonth == 0) {
            return 6;
        }

        return firstDayOfMonth - 1;
    }
    
    $: offsetDays = getOffsetDays();

    $: getMonthLength = () => {
        
        const FEBRUARY_INDEX = 1;

        if (actualMonth == FEBRUARY_INDEX) {
            return 28 + (actualYear % 4 == 0 ? 1 : 0);
        }

        if (actualMonth <= 6) {
            return 30 + (actualMonth % 2 == 0 ? 1 : 0);
        }

        return 30 + (actualMonth % 2 == 0 ? 0 : 1);
    }

    let projectsForDays: any[] = [];

    async function getProjectsForMonth() {
        projectsForDays = await invoke("get_projects_vector", {month: actualMonth, year: actualYear});
    }

    getProjectsForMonth();

    listen("added_project", () => {
        getProjectsForMonth();
    });

    listen("removed_project", () => {
        getProjectsForMonth();
    });

    listen("changed_month", () => {
        getProjectsForMonth();
    });
</script>

<div id="calendar-section">

    <div id="column-left">
        <DateSelector bind:month={actualMonth} bind:year={actualYear}/>

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
            {#if offsetDays != 0}
                <div style="grid-column-start: 1; grid-column-end: {offsetDays + 1}"></div>
            {/if}
            
            {#each [...Array(getMonthLength()).keys()] as day}
                <Day dayNumber={(day + 1).toString()} project={projectsForDays[day]}/>
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

