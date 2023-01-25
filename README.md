# Record thing

Thing to call api of Ovenmediaengine, to start/stop recording and check status

## You need to edit src/main.rs to set your domain and auth header. otherwise it wont work
### https api calls dont seem to work currently, didnt figure out why, yet.

### Build instructions:
```bash
cargo b -r
./target/release/record-thing-ome -h
```

### Usage:
```bash
./record-thing-ome -h
```

### For easier use you can put the binary into ".local/bin"
```bash
cp target/release/record-thing-ome ~/.local/bin
```

put an alias to ~/.local/bin/record-thing-ome in your .zshrc or .bashrc, then source it by doing:
```bash
source ~/.zshrc 
# or 
source ~/.bashrc
```
