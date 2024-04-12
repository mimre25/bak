# Bak - Simple file & directory backups
Have you ever been annoyed by typing `cp some_file some_file.bak`?
No more! 
Just use `bak some_file` and it will automatically create a copy with the `.bak` extension.

Have you ever run `mv some_directory.bak some_directory` just to find out that `some_directory` still exists and now you have `some_directory/some_directory.bak`?
No more!
Just use `unbak some_directory.bak` and you're good.

`bak` and `unbak` work for both files and directories the same way, with no hassling around.

## Installation
First, clone this git repository
```sh
git clone git@github.com:mimre25/bak.git
```
or
```sh
git clone https://github.com/mimre25/bak.git
```

Then, run 
```sh
cargo build --release
```
to build the binaries.

Last, copy the binaries from `target/release/bak` and `target/release/unbak` to some place on your path, or add `target/release` to your path.

## Anything else?
Nah, that's all. 
Just wanted to solve this little problem for myself and explore `rust` a bit on the way.


# Use at your own risk
