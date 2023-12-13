# Ecdar-GUI-Native

A native implementation of Ecdar-GUI-Web


### Install dependencies

Install `node`, `yarn`, `protobuf` and `cargo`

#### Linux
###### Arch
```
sudo pacman -S cargo nodejs protobuf yarn
```

###### Debian
```
sudo apt install cargo nodejs protobuf-compiler yarn
```

#### Mac
```
brew install protobuf node yarn
```

#### Windows
[Protoc](https://www.geeksforgeeks.org/how-to-install-protocol-buffers-on-windows/)
[Yarn](https://classic.yarnpkg.com/lang/en/docs/install/#windows-stable)
[Node](https://nodejs.org/en/download)
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)


### Cargo dependencies
```
cargo install tauri-cli
```
### Run
How to Ecdar-GUI-Native
#### Dev
```
cargo tauri dev
```
#### Release
```
cargo tauri build
```

###### Mac/Linux
The executable can be found at `./target/release/ecdar`
###### Windows
The executable can be found at `.\target\release\ecdar.exe`
