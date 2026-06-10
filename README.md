### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Test
```bash
cargo test 
```

### Example Usage 
```bash
$ cargo run
rshell$ ls
Cargo.lock  Cargo.toml  README.md  REQUIREMENTS.md  src  target
rshell$ ls $HOME; echo Hello, World
applications  desktop  documents  downloads  music  pictures  public  templates  videos
Hello, World
rshell$ exit

$ cargo build
$ ./target/debug/rshell -c "echo this is a test"
this is a test
$
```


#### Useful links
- https://mywiki.wooledge.org/BashGuide/SpecialCharacters
- https://www.gnu.org/software/bash/manual/html_node/Command-Grouping.html
