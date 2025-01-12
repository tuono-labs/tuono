[Documentation](https://tuono.dev) | [Tutorial](https://tuono.dev/documentation/tutorial) |
[Discord](https://discord.com/invite/khQzPa654B) | [✨Contributing](https://tuono.dev/documentation/contributing)

# Tuono

<img align="right" src="https://raw.githubusercontent.com/tuono-labs/tuono/main/assets/logo.png" width="160px">

![](https://github.com/tuono-labs/tuono/actions/workflows/rust.yml/badge.svg)
![](https://github.com/tuono-labs/tuono/actions/workflows/typescript.yml/badge.svg)

**Tuono is a full-stack web framework for building React applications using Rust as the backend with
a strong focus on usability and performance.**

Tuono (Italian word for "thunder", pronounced /2 Oh No/).
Why Tuono? Just a badass name.

If you have experience with [Next.js](https://nextjs.org/) you will feel home here.

Some of its features are:

- 🟦 Native Typescript
- 🌐 Next.js like Routing
- 🍭 CSS/SCSS modules
- 🧬 Server Side Rendering
- 🔥 Hot Module Reload

## 📖 Documentation

The [documentation](https://tuono.dev/) is available on
[tuono.dev](https://tuono.dev/).

## Introduction

**NodeJs/Deno/Bun are the only runtimes that allow a React app to be full-stack right? (no)**

Tuono is a full-stack React framework with the server side written in Rust.
Because of this, Tuono is extremely fast, and the requests are handled by a multithreaded Rust server.
React is still React - it is just superpowered.

**Rust is a hard language, then writing server-side code is hard as well, right? (no again)**

Tuono provides a collection of utilities to handle the server side code seamlessly with the React code.
Each server-side route is managed in a separate file alongside the React route. The routing is handled
by Tuono based on the files defined within the `./src/routes` directory.

**How is Tuono different from Next.js?**

The Tuono API tries to stick as much as possible to the Next.js one (or at least takes a huge inspiration
from it). The major difference is the backend system. While Next.js relies entirely on Node/Deno/Bun,
Tuono runs the server without any intermediary runtime. This enables impressive performance improvements
(check the benchmarks [here](https://github.com/tuono-labs/tuono-benchmarks)).

## Getting started

Tuono is the CLI that provides all the needed commands to handle the full-stack project.
To download it is required [cargo](https://doc.rust-lang.org/cargo/), which is the [rust](https://www.rust-lang.org/)
package manager.

Then run `cargo install tuono`.

To list all the available commands, run `tuono -h`

To create a new project, run `tuono new [NAME]` (optionally, you can pass the `--template` (or `-t`) flag - check the
[examples](https://github.com/tuono-labs/tuono/tree/main/examples) folder).

Then, to run the local development environment, install the JS dependencies (`npm install`) and run inside the project folder `tuono dev`

Finally, when the project is ready to be deployed, just run `tuono build` to create the final React assets
and to set the server project in `production` mode.

Now to execute it just run `cargo run --release`.

## Contributing

Any help or suggestion will be appreciated and encouraged.
Check the [✨Contributing](https://tuono.dev/documentation/contributing) page

## License

This project is licensed under the MIT License.
