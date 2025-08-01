# Contributing

## Project Structure

### Hexagon Architecture

Outgoing interfaces are implemented with ports and adapters. Incoming interfaces currently directly hardcode the application into an HTTP layer. If we needed to unit test this in the future we could convert it to an incoming port and adapter, but at the moment I feel that it hinders a developers parsing of the codebase.