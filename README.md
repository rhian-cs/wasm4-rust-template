# WASM-4 Game Template

This is the template that I (plan to) use for developing [WASM-4](wasm4.org/) games with Rust. It was extracted from my [rust-tutorial-raycaster](https://github.com/rhian-cs/rust-tutorial-raycaster), in which I followed a tutorial to create a Doom-like game and then refactored the code into multiple modules.

## Local Development

### Dependencies

- Install `make`
- Install Rust Nightly
- Install [`binaryen`](https://github.com/WebAssembly/binaryen)
  - For Ubuntu you can install it with `sudo apt install binaryen`
- Install wasm4 version 2.3.1
  - With ASDF you can run:
  ```sh
  asdf plugin-add wasm4
  asdf install wasm4 2.3.1
  ```
  - You'll probably need to install `glibc` version 2.31
    - For Ubuntu you can install it with `sudo apt install libc6`
- Enable the wasm architecture for Rust:
  - `rustup target add wasm32-unknown-unknown`

### Build and Run

Build with:

```
make
```

Build and run with:

```
make run
```

Build and check the size of the app with:

```
make size
```

Build and create a browser executable version of the game:

```
make bundle
```

### VS Code Tweaks

If you use VS Code and has the CodeLLDB extension installed, you can configure the workspace to run the app by pressing F5.

To do that just create a `.vscode/launch.json` directory and fill it with the following:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "program": "make",
      "args": ["run"],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

If your Rust Analyzer extension is annoying you about the panic handler, add the contents below to your local `config/settings.json` file ([source](https://github.com/rust-lang/rust-analyzer/issues/3801#issuecomment-1166269464)).

```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```

### Deployment

To deploy the file, I'm using the [gh-pages](https://github.com/tschaub/gh-pages) utility, which required Node (the `npx` executable in particular) to be installed.

Run:

```sh
make deploy
```

## License

[MIT License](https://choosealicense.com/licenses/mit/)
