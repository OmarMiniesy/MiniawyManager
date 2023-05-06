# Miniawy Manager

> _The_ Process management tool for Linux.

> Developed as both CLI and GUI.

# To Use the CLI

> Make sure to have rust and cargo installed.
> Refer to [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html)

#### BUILD

> To build the CLI, be inside the `/MiniawyManager/MiniawyManager` directory: `cargo build`

#### RUN

> To run the CLI, be inside the `/MiniawyManager/MiniawyManager` root directory: `cargo run`. To use flags: `cargo run -- -<flags>`.

#### EXECUTABLE

> To make it an executable to be run from anywhere:

1. `cargo build`
2. `cargo install --path .`

> Can now pass flags normally `MiniawyManager -<flags>`

#### HELP AND FLAGS

> Help Menu for the CLI (flags):

```
-T: Prints the processes in a tree format
-S <column>: Sorts the processes by the column provided
-F <column> <value>: Filters the processes by the column provided and the value provided
-FS <column> <value> <sort_column>: Filters the processes by the column provided and the value provided and then sorts the processes by the column provided
-P <R/D>: Prints the processes by the column provided
-A <pid>: Searches for the process with the pid provided
-N <name>: Searches for the process with the name provided
-O: Prints the overall system information and consumption
-K <pid>: Kills the process with the pid provided
-cP <pid> <priority>: Changes the priority of the process with the pid provided to the priority provided
-H: Prints the help menu
```

# To Use The GUI

#### DEPENDANCIES

> Make sure to have rust and cargo installed.
> Refer to [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html)

> Make sure to have tauri installed. Refer to [Tauri](https://tauri.app/).
> Use the `cargo` installation technique.

> Make sure to have NodeJs installed. Refer to [nodeJs](https://nodejs.org/en).

> Make sure to have npm installed. Refer to [npmJs](https://www.npmjs.com/).

> Make sure to have reactJs installed. Refer to [reactJs](https://react.dev/).

> This project uses [Recharts](https://recharts.org/en-US/) and [Material React Table](https://www.material-react-table.com).

#### RUN DEVELOPMENT ENVIRONMENT

> To run this project, make sure to be inside `MiniawyManager/MiniawyManagerGui` directory.

> Before running, install suitable dependancies with `npm i`. If any errors are faced, update the version of `node` or `npm`. This is done inside the `MiniawyManager/MiniawyManagerGui/src` directory.

> To run and view the development environment be in `MiniawyManager/MiniawyManagerGui` directory and enter `npm run tauri dev`.

### BUILD EXECUTABLE

> Must first build the project to be able run the executable directly.

> Navigate to `MiniawyManager/MiniawyManagerGui` and run `npm run tauri build`. If that doesn't work, try in `MiniawyManager/MiniawyManagerGui/src`.

> It will take some time, and it will place the executable in the following directory: `MiniawyManager/MiniawyManagerGui/src-tauri/target/release/` and the file is called `miniawy-manager-gui`.

> Give it the correct permissions to execute via `chmod`, and then run it via: `./miniawy-manager-gui`.

### MANAGE CARGO AND RUST DEPENDANCIES

> If some backend erros result due to missing libraries or unkown imports, navigate to `MiniawyManager/MiniawyManagerGui/src-tauri` and check the `cargo.toml` file.

> Inside the same directory, add the missing dependancies via `cargo add <dependancy-name>`.

### KILLING A PROCESS IN THE GUI

> If the filters and/or searching was used to locate a process to kill it, must close the filter/search tab after finding the process required for the kill button to become interactive. 

# Versions Used At Time of Development

```

rust: 1.69.0
cargo: 1.69.0
tauri: 3.3.6 (create-tauri-app)
node: 20.0.0
npm: 9.6.5
react: 18.2.0

//crates (cargo.toml file)
fs : 0.0.5
libc : 0.2.141
procfs : 0.15.1
sysinfo : 0.28.4
users : 0.11.0

//node modules (pacakge.json file)
recharts: 2.5.0
react: 1.11.2

```
