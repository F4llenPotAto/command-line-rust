How do we read the opened file line by line?
* Start with tests/inputs/fox.txt

```bash
cargo run -- tests/inputs/fox.txt
```

Verify that you can read STDIN by default.

```bash
cat tests/inputs/fox.txt | cargo run
```

The output should be the same when providing a dash as a 
filename.

```bash
cargo run -- - < tests/inputs/fox.txt
```

