# create-yew-app
This package includes the global command for [Create Yew App](https://github.com/jetli/create-yew-app). <br/>
Please refer to its documentation:

- [Quick Overview](https://github.com/jetli/create-yew-app#quick-overview) – How to create a new app.
- [Creating an App](https://github.com/jetli/create-yew-app#creating-an-app) – How to develop apps bootstrapped with Create Yew App.

## Getting Started

### Install Rust wasm target

```sh
rustup target add wasm32-unknown-unknown
```

### Install [Trunk](https://trunkrs.dev)

### Creating an App

```sh
npx create-yew-app my-app
cd my-app
trunk serve
```

Then open [http://localhost:8080/](http://localhost:8080/) to see your app.<br/>
When you’re ready to deploy to production, create a minified bundle with `trunk build`.
