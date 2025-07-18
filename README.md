# Potree Auth

## Installation

### Rust Binary

If you have Rust installed you can build and install `potree-auth` by running the following from the root of the project.

```bash
cargo install --path .
```

### Docker

You can also build a Docker container by running the following from the root of the project.

```bash
make docker-build
```

## Usage

### Configuration

Various options are available either by command line arguments or as environment variables (or a mixture of the two).

The command line arguments are most accessible if running the application from the binary. The various options can be viewed by running the following.

```bash
potree-auth --help
```

To the right of the CLI argument, the name in angle brackets (`<>`) indicates the environment variable representing the argument. If the environment variable is set, this will be used without having to specify it in the CLI arguments.

`potree-auth` will also respect a `.env` file in the directory from which the application is called. An example `.env` file is available [here](example.env).

### Docker

A Docker Compose file is provided ([here](./docker-compose.yml)) to provide the necessary arguments to run the container. This can be run with the following. It will also respect a `.env` file.

```bash
make docker-run
```

## Adding Projects

### Project Configuration

#### Project Directory

Contains all the data for a project.

#### Manifest File

#### Potree Config

When a project is loaded the application will look for a `potree.json5` file ([example](./docs/resources/potree.json5)) in the root of the [project directory](#project-directory). It configures which data potree should load and how to display it.

### Project Data

### Recommended Project Structure.

### Viewing a Project