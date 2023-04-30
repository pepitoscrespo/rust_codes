# rust_codes

# Rust Large File Sorting

This Rust project efficiently sorts large files by splitting them into smaller chunks, sorting each chunk in parallel, and then merging the sorted chunks into a single output file.

## Project Structure

The project has the following structure:

rust_codes/
   Cargo.toml
   README.md
   src/
     -- main.rs
     -- sortFile.rs


## Dependencies

This project uses the following dependencies:

- `tokio`: for asynchronous file I/O operations
- `rayon`: for parallelism
- `futures`: for working with asynchronous operations

Make sure to add these dependencies to your `Cargo.toml` file.

## Build and Run

To build and run the project, follow these steps:

1. Clone the repository to your local machine.

git clone https://github.com/pepitoscrespo/rust_codes.git

cd rust_codes

2. Build the project using Cargo.

3. Run the project, specifying the input file and output file as command-line arguments. cargo run --release -- <input_file> <output_file>

Replace `<input_file>` with the path to the large file you want to sort, and `<output_file>` with the desired path for the sorted output file.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for improvements, bug fixes, or new features.

## License

This project is released under the [MIT License](https://opensource.org/licenses/MIT).
