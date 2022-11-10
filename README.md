# discorder

## useful links
- Discord user data description: https://support.discord.com/hc/en-us/articles/360004957991

## How to build from sources and run
1. request a copy of your discord data. to request: https://support.discord.com/hc/en-us/articles/360004027692-Requesting-a-Copy-of-your-Data
2. download and unzip your discord data
3. make sure that you have installed cargo. to install: https://doc.rust-lang.org/cargo/getting-started/installation.html
4. clone this repository
5. `cd discorder`
6. `cargo run {path to folder with your discord data}` or `cargo build -r` and then run it as a native app (build output can be found in ./target/release/ folder)
