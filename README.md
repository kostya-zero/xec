## Xec

Xec is a command executor that executes commands in scriptlets.
It's allows you to automate your workflow for better productivity.

## Installation

Xec can be compiled from source.
You need to install Rust compiler and make sure that you have `cargo`.
Now, clone the repo, enter the repo directory and call `cargo` to build application.

```shell
cargo build
```

> You also can use `RUSTFLAGS`.

## Configuration

Xec configuration file has JSON structure. File has name `xec.conf`. Here is example:

```json
{
    "scriptlets": {
        "build": [
            "dotnet restore",
            "dotnet build"
        ],
        "run": [
            "dotnet run"
        ]
    }
}
```

## Usage 

To run scriptlet you need to specify them in config.
Then, use run command and give scriptlet name.

```shell
xec run build
```