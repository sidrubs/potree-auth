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

#### Docker, TODO: UPDATE WITH CORRECT DOCKER IMAGE NAMES.

Requires the [Docker Engine](https://docs.docker.com/engine/) to be installed.

```bash
docker run -p 3000:3000 -v /<path-to>/project-data:/project-data -e DATA_DIR="/project-data" -e SERVER_HOST="0.0.0.0" potree-auth:latest
```

Can also be run using Docker Compose with [this example `docker-compose.yml` file](./docs/resources/docker-compose.yml).

```bash
DATA_DIR=/<path-to>/project-data docker compose -f /<path-to>/docker-compose.yml up
```

## Application Overview

`potree-auth` sits in front of your `potree` projects and makes them easy to manage, secure, and serve.
Here are the main pieces that work together:

1. **Projects**
   A *project* is a collection of point-cloud data plus a simple [`manifest.yml`](./docs/resources/manifest.yml) file that describes it. Each project lives in its own folder inside your data directory, and access is controlled per project.

2. **Project Asset Server**
   Serves files (point clouds, metadata, etc.) from each project directory — but **only** to users who are authorized for that project. Assets are available at: `/project-assets/{project_id}/{*path}`

3. **Potree Asset Server**
   Serves the standard Potree viewer files (JavaScript, CSS, etc.) that don’t require authentication. Available at: `/potree-assets/{*path}`

4. **Potree Rendering Template**
   A pre-configured Potree HTML template is provided so you can spin up visualizations quickly. Each `potree` project define its rendering properties in a standard [`potree.json5`](./docs/resources/potree.json5) file. Access is restricted to authorized users at: `/potree/{project_id}`

5. **Project Dashboard**
   The home page for users. After logging in, they’ll see a clean dashboard listing all the projects they can access — nothing more, nothing less. Available at: `/projects`

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

> **Note:** Project directory names should be URL safe as they are used as the `project_id` in the URL.

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
