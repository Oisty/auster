<h1 align="center" style="border-bottom: none">
    <a href="https://github.com/Oisty/Auster" target="_blank"><img alt="Auster" width="120px" src="https://github.com/Oisty/Auster/blob/main/assets/logo/logo.svg"></a><br>Auster
</h1>
<p align="center">Auster is a webservice for managing containerized gameservers with a simple, powerful interface</p>

<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

</div>

> :warning: Until the first development cycle is complete, please do not use Auster in any capacity. You will not receive any support for this project, as it is heavy in development.

## 🚀 Get Started

> 🚧 There is currently no way to easily get started with running Auster as a user. We will add a Get Started when we have released the first runnable verion. 🚧

## 👷 Get Started (Development)

_For frontend devs to start the backend and backend devs to start the frontend_

### Frontend

1. Install [Node.js](https://nodejs.org)
2. Clone this repository
3. Go to `src/web`
4. Run `npm install` to install all dependencies
5. Run `npm run dev` to run the vite dev server for the frontend. Alternatively you can `npm run build` to build it and host it with the backend.

### Backend

ℹ️ If you want to see the front end hosted by the backend you need to run the steps for the frontend above first

1. Install the [Rust](https://rust-lang.org) programming language toolchain
2. Switch to Rust nightly version with

   ```
   rustup default nightly
   rustup update
   ```

3. Install [Docker](https://docker.com)
4. Clone this repository
5. From the repository go to the `deployments` directory
6. Run `docker compose up -d`. Then you're ready to run the API
7. You find the API in `src/api` and can start it from there with `cargo run`

## Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

1. Submit a feature request or bug report as an [issue].
2. Ask for improved documentation as an [issue].
3. Comment on [issues that require feedback].
4. Contribute code via [pull requests].

[issue]: https://github.com/Oisty/Auster/issues
[issues that require feedback]: https://github.com/Oisty/Auster/issues?q=is%3Aissue+is%3Aopen+label%3A%22feedback+wanted%22
[pull requests]: https://github.com/Oisty/Auster/pulls

We aim to keep Auster's code quality at the highest level. This means that any
code you contribute must be:

- **Commented:** Complex and non-obvious functionality must be properly
  commented.
- **Documented:** Public items _must_ have doc comments with examples, if
  applicable.
- **Styled:** Your code's style should match the existing and surrounding code
  style.
- **Simple:** Your code should accomplish its task as simply and
  idiomatically as possible.
- **Tested:** You must write (and pass) convincing tests for any new
  functionality.
- **Focused:** Your code should do what it's supposed to and nothing more.

All pull requests are code reviewed and tested by the CI. Note that unless you
explicitly state otherwise, any contribution intentionally submitted for
inclusion in Auster by you shall be licensed Apache License, Version 2.0,
without any additional terms or conditions.

## ⚖️ License

Auster is licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))

The Auster website source is licensed under separate terms.

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/Oisty/Auster.svg?style=for-the-badge
[contributors-url]: https://github.com/Oisty/Auster/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Oisty/Auster.svg?style=for-the-badge
[forks-url]: https://github.com/Oisty/Auster/network/members
[stars-shield]: https://img.shields.io/github/stars/Oisty/Auster.svg?style=for-the-badge
[stars-url]: https://github.com/Oisty/Auster/stargazers
[issues-shield]: https://img.shields.io/github/issues/Oisty/Auster.svg?style=for-the-badge
[issues-url]: https://github.com/Oisty/Auster/issues
[license-shield]: https://img.shields.io/github/license/Oisty/Auster.svg?style=for-the-badge
[license-url]: https://github.com/Oisty/Auster/blob/master/LICENSE.txt
