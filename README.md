# Project Calendar

A small desktop app made with Tauri to manage your projects in an easy and comfortable way.

# To modify it
Make sure you have a stable [Rust](https://www.rust-lang.org/tools/install) and [Node](https://nodejs.org/en) install.

```bash
# Clone the repo
git clone https://github.com/JatoMixo/CalendarApp.git

# Install the dependencies for the frontend
npm install

# Install the tauri cli
cargo install tauri-cli

# To start a dev window
cargo tauri dev

# To build the application
cargo tauri build
```

# UI Components

## Date Selector
This one is the small bar you see on the top of the app to change the month/year the user is visualizing.

It has variables:
 - `month`: To store the month by index going from 0 to 11 (January - December)
 - `year`: To store the actual year
 - `MONTHS`: List with the months to access through index

And methods:
 - `NextMonth()`: Asigned to the right button to move the the next month
 - `LastMonth()`: Asigned to the left button to go back a month

## Add Project
This is a small card used to add a new project to the list.

Variables:
 - `name`: The name of the new project binded to an input of type text
 - `description`: The description of the new project binded to an input of type text
 - `start_date`: The initial date of the new project
 - `final_date`: The deadline of the new project

And the method `AddProject()` to add the project to the list

## Day
Small card used to show a day with its projects.

Variables:
 - `project`: The project that the day contains (undefined if it doesn't contain one)
 - `day_number`: The day shown top left.