# Ecdar-GUI-Native

A native implementation of Ecdar-GUI-Web


### Install dependencies

Install `node`, `yarn`, `protobuf` and `cargo`

#### Linux

###### Debian
```
sudo apt install cargo nodejs protobuf-compiler yarn
```

###### Arch
```
sudo pacman -S cargo nodejs protobuf yarn
```


#### Mac
```
brew install protobuf node yarn
```

Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Windows
Install [Protoc](https://www.geeksforgeeks.org/how-to-install-protocol-buffers-on-windows/)

Install [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/#windows-stable)

Install [Node](https://nodejs.org/en/download)

Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)


### Cargo dependencies
To run tauri applications some need to install tauri cli
```
cargo install tauri-cli
```
### Run
How to run Ecdar-GUI-Native
#### Dev
```
cargo tauri dev
```
#### Release
```
cargo tauri build
```
In release the executable can be found at
###### Mac/Linux
```./target/release/ecdar```
###### Windows
```.\target\release\ecdar.exe```
