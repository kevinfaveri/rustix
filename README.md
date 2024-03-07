# Rustix WebApp
### Rust Rest API + Remix FrontEnd
This is a fullstack web application built with Rust and TypeScript.

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

- Rust
- Cargo
- Node.js
- npm or yarn

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/kevinfaveri/rustix.git
   ```
2. Navigate into the project directory
   ```sh
   cd rustix
   ```
3. Install NPM packages for the frontend
   ```sh
   cd frontend && npm install
   ```
4. Build and run the Rust backend
   ```sh
   cd ../backend && cargo watch -x run
   ```
5. In another terminal, run the tests for the Rust backend
   ```sh
   cd ../backend && cargo nextest run
   ```

## Usage

This boilerplate is designed to kickstart development of fullstack web applications using Rust for the backend and TypeScript with Remix for the frontend. Modify and extend it to suit the needs of your project.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Kevin Faveri - [@kevfaveri](https://twitter.com/kevfaveri)

Project Link: [https://github.com/kevinfaveri/rustix](https://github.com/kevinfaveri/rustix)

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Remix](https://remix.run/)
- [RustAPI](https://github.com/ndelvalle/rustapi)

## Roadmap
BE REST API:
- Add support for postgresql (with models and queries, replacing all mongo related dependencies; and fixing all tests) on Rust API
- Add Redis support for caching
- Add support for CORS on Rust API
- Add support for dockerfile for BE
- Add support for docker-compose with postgres connected to the dockerfile of BE
- Add support for Diesel ORM on Rust API
- Add support DB migration solutions (study crates for the purpose)
- Do pair review on all dependencies used; if everything is good practices, and if there is a better way to do it

FE:
- Add Remix with tailwind config, prettier, eslint
- Add docker-compose for running both FE with hot reload
