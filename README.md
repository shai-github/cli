# Command Line Program in Rust

This is a version of the CLI App outlined in [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html) by Steve Klabnik and Carol Nichols. 

### Running the program

To run the program, you can use the `sample.txt` file or use your own text file. Run the following command to check for substrings across all lines in the text file and save valid results in an output text file:

```
cargo run <query> <text file> > output.txt
```

For example, you can run the following:

```
cargo run to sample.txt > output.txt
```

This will result in the following results in the `output.txt` file

```
Searching for to in file sample.txt
All the great books of the past and all the ones yet to come are made with these words. 
All you must learn to do is to use them well and in the right places.
```