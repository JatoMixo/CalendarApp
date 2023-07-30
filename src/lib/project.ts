type Project = {
    name: string,
    color: string,
    description: string,

    start_date: {
        day: number,
        month: number,
        year: number,
    },

    final_date: {
        day: number,
        month: number,
        year: number,
    }
};