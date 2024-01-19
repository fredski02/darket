# Darket



## Dependencies


```sh
rustup target add wasm32-unknown-unknown # Required for building IC canisters
cargo install cargo-watch # Optional; used for live reloading in `npm start`
```

Next, run the following commands in a new, empty project directory:

```sh
npm run setup # Install packages, deploy canisters, and generate type bindings

npm run frontend # Start the development server
```

When ready, run `dfx deploy` to build and deploy your application.

## Technology Stack

- [Vite](https://vitejs.dev/): high-performance tooling for front-end web development
- [Vue](https://reactjs.org/): a component-based UI library
- [TypeScript](https://www.typescriptlang.org/): JavaScript extended with syntax for types
- [Sass](https://sass-lang.com/): an extended syntax for CSS stylesheets
- [Prettier](https://prettier.io/): code formatting for a wide range of supported languages
- [Rust](https://www.rust-lang.org/): a fast, safe programming language for writing [Internet Computer](https://internetcomputer.org/) canisters
