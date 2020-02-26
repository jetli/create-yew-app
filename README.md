# Create Yew App

Create Yew apps with no build configuration.

- [Creating an App](#creating-an-app) – How to create a new app.
- [Yew](https://github.com/yewstack/yew) - Rust / Wasm framework for building client web apps.
- [Awesome Yew](https://github.com/jetli/awesome-yew) - A curated list of awesome things related to Yew / WebAssembly.

[Check out a live demo](https://jetli.github.io/create-yew-app/) powered by Create Yew App.

## Quick Overview

```sh
npx create-yew-app my-app
cd my-app
npm start
```

_([npx](https://medium.com/@maybekatz/introducing-npx-an-npm-package-runner-55f7d4bd282b) comes with npm 5.2+ and higher, see [instructions for older npm versions](https://gist.github.com/gaearon/4064d3c23a77c74a3614c498a8bb1c5f))_

Then open [http://localhost:8000/](http://localhost:8000/) to see your app.<br/>
When you’re ready to deploy to production, create a minified bundle with `npm run build`.

<p align='center'>
    <img src="packages/create-yew-app/demo.svg" />
    
_(demo.svg is created by [termtosvg](https://github.com/nbedos/termtosvg))_
</p>

### Get Started Immediately

You **don’t** need to install or configure tools like Webpack or Parcel.<br>
They are preconfigured and hidden so that you can focus on the code.

Create a project, and you’re good to go.

## Creating an App

**You’ll need to have Node 8.16.0 or Node 10.16.0 or later version on your local development machine** (but it’s not required on the server). You can use [nvm](https://github.com/creationix/nvm#installation) (macOS/Linux) or [nvm-windows](https://github.com/coreybutler/nvm-windows#node-version-manager-nvm-for-windows) to switch Node versions between different projects.

To create a new app, you may choose one of the following methods:

### npx

```sh
npx create-yew-app my-app
```

_([npx](https://medium.com/@maybekatz/introducing-npx-an-npm-package-runner-55f7d4bd282b) is a package runner tool that comes with npm 5.2+ and higher, see [instructions for older npm versions](https://gist.github.com/gaearon/4064d3c23a77c74a3614c498a8bb1c5f))_

### npm

```sh
npm init yew-app my-app
```

_`npm init <initializer>` is available in npm 6+_

### Yarn

```sh
yarn create yew-app my-app
```

_[`yarn create <starter-kit-package>`](https://yarnpkg.com/lang/en/docs/cli/create/) is available in Yarn 0.25+_

### Project structure

It will create a directory called `my-app` inside the current folder.<br/>
Inside that directory, it will generate the initial project structure and install the transitive dependencies:

```
my-app
├── README.md
├── node_modules
├── package.json
├── .gitignore
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── static
│   ├── favicon.ico
│   ├── index.html
│   ├── index.ts
│   ├── logo.svg
│   └── styles.css
└── src
    ├── lib.rs
    ├── app.rs
    ├── components
    │   ├── nav.rs
    │   └── mod.rs
    └── routes
        ├── about.rs
        ├── home.rs
        └── mod.rs
```
No configuration or complicated folder structures, only the files you need to build your app.<br>
Once the installation is done, you can open your project folder:

```sh
cd my-app
```

Inside the newly created project, you can run some built-in commands:

### `npm start` or `yarn start`

Runs the app in development mode.<br>
Open [http://localhost:8000](http://localhost:8000) to view it in the browser.

The page will automatically reload if you make changes to the code.<br>
You will see the build errors and lint warnings in the console.

### `npm test` or `yarn test`

Runs the test watcher in an interactive mode.<br>
By default, runs tests related to files changed since the last commit.

### `npm run build` or `yarn build`

Builds the app for production to the `dist` folder.<br>
It correctly bundles Yew in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br>

Your app is ready to be deployed.

# Contributing

Feel free to take a look at the current issues in this repo for anything that currently needs to be worked on.

You are also welcome to open a PR or a new issue if you see something is missing or could be improved upon.

# License

Apache License (Version 2.0)

See [LICENSE](./LICENSE) for details.
