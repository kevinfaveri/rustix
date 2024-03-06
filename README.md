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
   cd ../backend && cargo run
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

Kevcode - [@kevcode_art](https://twitter.com/kevcode_art)

Project Link: [https://github.com/kevinfaveri/rustix](https://github.com/kevinfaveri/rustix)

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Remix](https://remix.run/)
- [RustAPI](https://github.com/ndelvalle/rustapi)

## Roadmap
- Add support for postgresql on Rust API
- Add support for prettier and rust analyzer / clip
- Add Remix with tailwind config, prettier, eslint
- Add docker-compose for running both services with hot reload
