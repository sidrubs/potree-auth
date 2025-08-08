# Contributing

## Project Structure

The project uses [hexagon architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)). Outgoing interfaces are implemented with ports and adapters. Incoming interfaces currently directly hardcode the application into an HTTP layer. If we needed to unit test this in the future we could convert it to an incoming port and adapter, but at the moment I feel that it hinders a developers parsing of the codebase.

Within the [src](./src) directory are the following child directories:

- [application_lib](./src/application_lib): Composes the functionality from the rest of the source code. Called and run directly from `main.rs`.
- [authentication](./src/authentication): Contains OIDC authentication domain code.
- [common](./src/common): Contains the code common to multiple domains.
- [potree_assets](./src/potree_assets): Contains the code to serve potree related assets.
- [project_assets](./src/project_assets): Contains the code to serve project assets (e.g. point clouds, etc.)
- [render](./src/render): Contains the code to render HTML pages.

## Testing

Some very high-level integration tests are in the [tests](./tests) directory. This is really just to check that the application has been wired up correctly. More thourough tests should be implemented for the child routers with mocked ports, or as unit tests.
