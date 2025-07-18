# Potree Auth

## Application Overview

The core components of the application are:

1. Project
2. Authenticated project asset server
3. Potree asset server
4. Pre-configured potree rendering template

### Project

Groups data and manages access to it. Project metadata is defined in the [manifest file](./docs/resources/manifest.yml).

A project is created by placing a YAML-formatted `manifest.yml` (note: use `.yml`, not `.yaml`) in a subdirectory of the _project data_ directory (as specified in the application config).

The subdirectory (_project directory_) name serves as the `project_id` and should be URL-safe — `kebab-case` is recommended.


### Project Asset Server

Provides authenticated access to files within a _project directory_, identified by `project_id`. Access is granted only to users belonging to at least one of the project's groups.

Assets are served at `/project-assets/{project_id}/{*path}`.


### Potree Asset Server

Serves standard [`potree`](https://github.com/potree/potree) assets. No authentication required.

Served at `/potree-assets/{*path}`.


### Potree Rendering Template

A pre-configured Potree HTML template ([example](./templates/potree_render.html)) is available. It loads settings from a [`potree.json5`](./docs/resources/potree.json5) file in the root of the project's directory (`project_id`).

Access requires the user to belong to at least one of the [project](#project) groups.

Served at `/potree/{project_id}`.

> To use custom Potree HTML, create it in an `index.html` file and add it to the project's directory. Access it via `/project-assets/{project_id}/index.html`.


## Installation

### Rust Binary

If you have Rust installed you can build and install `potree-auth` natively.

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

#### Building

```bash
cargo install --path .
```

### Docker

You can also build a Docker container by running the following from the root of the project.

#### Prerequisites

- [Docker](https://www.docker.com/)

#### Building

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
