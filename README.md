# rocket-template

This repository contains a templete to quickly create a web project based on the [Rocket framework](https://rocket.rs/) + [Askama](https://github.com/djc/askama) with [TailwindCSS](https://tailwindcss.com/) for writing the frontend.

More content will be added in wikis in the future.

## Requirements
- [Rustup](https://rustup.rs/) installed (1.23.0+).
- [NodeJS](https://nodejs.org/en/download/) (11+)

## Usage
1. Clone the repo.
1. Under the main folder execute `cargo run`.
1. Navigate to `localhost:8000`.

## Structure
The project is divided into 2 main folders: **public** and **src**.
- **public**: Here is all the frontend. Since Askama is being used, is recommended to read the [Askama book](https://djc.github.io/askama/template_syntax.html). TailwindCSS can be used.
- **src**: Backend code and backend <-> frontend linkage.