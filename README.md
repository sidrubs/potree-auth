# Potree Auth

Easily share and manage your 3D point cloud projects — securely.

`potree-auth` is a web server that adds authentication, access control, and a clean dashboard on top of [`potree`](https://github.com/potree/potree). Users log in with the configured Identity Provider (OIDC supported) and see only the projects they’re authorized to view.

With Potree Auth you get:
- 🔐 Secure access control for your Potree projects
- 🖥️ A ready-to-use project dashboard for your users

## Quick Start

### Installation

The two suggested methods to run `potree-auth` are installing with `cargo` or via Docker.

Both these examples make use of an example project directory that can be downloaded from [here](https://sidrubs.github.io/potree-auth-example-data/project-data.zip) and unzipped. More details on the structure of the project directory and the authentication configuration can be seen in the [**Usage**](#usage) section.

#### Cargo

Requires the [Rust toolchain to be installed](https://www.rust-lang.org/tools/install) on your system.

```bash
# Install `potree-auth`.
cargo install potree-auth

# Run `potree-auth` indicating where it can find the project data directory.
potree-auth --data-dir /<path-to>/project-data
```

Navigate to [http://localhost:3000](http://localhost:3000).

#### Docker

Requires the [Docker Engine](https://docs.docker.com/engine/) to be installed.

```bash
docker run -p 3000:3000 -v /<path-to>/project-data:/project-data -e DATA_DIR="/project-data" -e SERVER_HOST="0.0.0.0" potree-auth:latest
```

Can also be run using Docker Compose with [this example `docker-compose.yml` file](./docs/resources/docker-compose.yml).

```bash
DATA_DIR=/<path-to>/project-data docker compose -f /<path-to>/docker-compose.yml up
```

## Application Overview

The core components of the application are:

1. Project
2. Authenticated project asset server
3. Potree asset server
4. Pre-configured potree rendering template
5. Project dashboard

### Project

Groups data and manages access to it. Project metadata is defined in a [manifest file](./docs/resources/manifest.yml).

A project is created by placing a YAML-formatted `manifest.yml` (note: use `.yml`, not `.yaml`) in a subdirectory of the _data directory_ (as specified in the [application config](#configuration)).

The subdirectory (_project directory_) name serves as the `project_id` and should be URL-safe — `kebab-case` is recommended.


### Project Asset Server

Provides authenticated access to files within a _project directory_, identified by `project_id`. Access is granted only to users belonging to at least one of the project's groups.

Assets are served at `/project-assets/{project_id}/{*path}`.


### Potree Asset Server

Serves standard [`potree`](https://github.com/potree/potree) assets. No authentication required.

Served at `/potree-assets/{*path}`.


### Potree Rendering Template

A pre-configured Potree HTML template ([example](./templates/potree_render.html)) is available. It loads settings from a [`potree.json5`](./docs/resources/potree.json5) file in the root of the _project directory_ (`project_id`).

Access requires the user to belong to at least one of the [project](#project) groups.

Served at `/potree/{project_id}`.

> To use custom Potree HTML, create it in an `index.html` file and add it to the _project directory_. Access it via `/project-assets/{project_id}/index.html`.


### Project dashboard

Displays all the projects that a user has authorization to read.

Served at `/projects`.


## Usage

### Data Directory

A directory containing all the [_project directories_](#project) should be set up. An example directory structure is shown below.

```
.
└── data-dir/
    ├── project-1/
    │   ├── manifest.yml
    │   ├── potree.json5
    │   └── point-cloud/
    │       ├── file-one.bin
    │       └── file-two.bin
    └── project-2/
        ├── manifest.yml
        ├── potree.json5
        └── point-cloud/
            ├── file-one.bin
            └── file-two.bin
```

### Configuration

Configuration options can be set via command-line arguments, environment variables, or a mixture of both.

Command-line arguments are most convenient when running the application from the binary. To view available options, run:

```bash
potree-auth --help
```
Each CLI argument has a corresponding environment variable, shown in angle brackets (`<>`). If the environment variable is set, it overrides the need to specify the CLI argument.

`potree-auth` also supports a `.env` file in the current working directory. An example is available [here](example.env).

For authentication-specific settings, see the [Authentication section](#authentication).

### Authentication

Authentication is handled via the OIDC Authorization Code flow, supported by most modern Identity Providers (IdPs). Relevant configuration parameters are prefixed with `idp-`. If these values are not set, authentication is disabled and all users are granted access to all projects.

> **Note:** Users in the `admin` group have full access to all projects, even if `admin` is not explicitly listed in the project metadata.

## Development

Prerequisites:

- Rust stable
- Rust nightly (optional, for formatting)
- Docker (optional, for building Docker containers)

The [Makefile](./Makefile) contains commonly used commands during development for reference.
