# Record thing

Thing to call api of Ovenmediaengine, to start/stop recording and check status

### Set your api token and domain in stuff.conf (You need to put the values in quotes, like this: "http://pingusmc.org:8081" (or other configured port). The same for the token.
### Do NOT remove 'Basic' from stuff.conf at token: "Basic TOKEN"! Otherwise, it will NOT work
#### https api calls don't seem to work currently, didn't figure out why, yet.

#### For more information on how to get the token and how to change the api port, check the [Ovenmediaengine Documentation](https://airensoft.gitbook.io/ovenmediaengine/rest-api)

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

put an alias to ~/.local/bin/record-thing-ome in your .zshrc or .bashrc:
```bash
alias ome-rec="~/.local/bin/record-thing-ome"
```

config location stuff is a bit retarded at the moment, in the meantime you can have something like this in your .zshrc or .bashrc:
```bash
bash -c "cd /home/passihd/.config/ome-rec; /home/passihd/.local/bin/record-thing-ome -h"
```

then source it by doing:
```bash
source ~/.zshrc 
# or 
source ~/.bashrc
```
