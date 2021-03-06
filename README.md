# Markdown to Qase Converter

Repository containing code for site: https://mdtoqase.eejalab.xyz/

This is a browser-only tool to convert test cases written in Markdown format to [Qase](https://qase.io/) tool's JSON format.

## Overview

- [Core converter logic](https://github.com/balajeerc/md-qase-converter-ui/tree/main/rust) is written in Rust and used on the page via WASM
- The UI for the page is created using [plasmic](https://www.plasmic.app/)
- [NextJS](https://nextjs.org/) framework forms the base boilerplate on which this app is built

## Getting Started

### Quickstart with Docker

You can run the app with Docker:

```
docker build -t md-qase-convert . && docker run -p 3000:3000 md-qase-convert
```

This should have the app running at `localhost:3000`

### Pre-requisites

__Rust Toolchain__

You need the following installed:

- [rustup](https://rustup.rs/)
  - This will in turn install `rustc` and `cargo`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

__NodeJS__

You need a rather recent version of NodeJS. Recommended: v14.17.6 LTS. (Use [nvm](https://github.com/nvm-sh/nvm) to install it). 

__Plasmic Editor Access__

Plasmic let me create this UI without having to write any CSS myself (I suck at it).
Please ping me if you need access to the [plasmic project](https://studio.plasmic.app/projects/sDj2ruLAFhmZZDvF4NRj4P) that this was made from.

### Running Development Server

First build the Rust WASM

```
(cd rust && wasm-pack build)
```

Then run dev server:

```bash
npm install
npm run dev
```

### Updating Plasmic UI Edits

You need to install [plasmic-cli](https://docs.plasmic.app/learn/cli/) to do this.
Changes made in Plasmic can be transmitted to code here using one of the following facilities that `plasmic-cli` gives:

- Install plamsmic-cli using:

  ```
  npm install -g @plasmicapp/cli
  ```

- Auth to plasmic using:

  ```
  plasmic auth
  ```

- For one time updates:

  ```
  plasmic sync
  ```

- For realtime updates from Plasmic editor to local test env during development:

  ```
  plasmic watch
  ```
