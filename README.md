# TriPPP WebApp
### Rust Rest API + Remix FrontEnd
This is a fullstack web application built with Rust and TypeScript.

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

- Rust
- Cargo
- Node.js
- npm or yarn
- Docker and Docker Compose

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/kevinfaveri/trippp.git
   ```
2. Navigate into the project directory
   ```sh
   cd trippp
   ```
3. TODO: Ignore: Install NPM packages for the frontend
   ```sh
   cd frontend && npm install
   ```
4. Start the backend services with Docker Compose
   ```sh
   cd ../backend && docker-compose up --build -d
   ```
5. Build and run the Rust backend using Cargo Watch
   ```sh
   cargo watch -x run

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

Project Link: [https://github.com/kevinfaveri/trippp](https://github.com/kevinfaveri/trippp)

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Remix](https://remix.run/)
- [RustAPI](https://github.com/ndelvalle/rustapi)

## Roadmap
BE REST API:
- Fully test auth; make it ready for FE too
-> To make it ready, use oauthredirecturi to test; and check with Rapid API that cookie gets correctly set for other requets; auth requests start working; etc
-> Tests that logout works as expected by removing cookie, sesssion
-> Tests that expiration works as expected if user don't logout
-> Test that user data gets correctly saved (picture, name, etc) to the DB
-> Add support for permission/roles in the DB, by using the user table to validate this stuff; check_auth_for_role gets added, too, then gate all user routes
-> Add per user gated boxes by adding a check_auth_for_user (which tests the box is from that user, and only that user can access it)
- Add Redis support for caching
- Add support for CORS on Rust API to be production ready
- Check notion for adding the recipes / AI integration module / package / and scrapper of web data

FE:
- Add Remix with tailwind config, prettier, eslint
- FE should have a route / monitor if it gets a couple query params like: IS_SUCCESS_LOGIN and then proxy all query params to the /oauth_return in the BE; or use some global variable / callback, that is waited on the login page
-> This will be proof of concept of the recipe
-> If user is not authenticated, the haiku bot will never get prompt injected; and will only provide a login option, saying: "You are not logged yet, please click Login with Google below for us to begin. I can't help you if you don't login"
- After that, FE waits the turn of /oauth_return which is a new cookie to be set on the browser. Setting the cookie, all is good
- Logout should work correctly removing the cookie too