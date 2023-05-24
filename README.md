# CSV to Embedding Vectors

This repository contains a script that takes a CSV file and converts each row into a text embedding vector using the OpenAI API.

The script reads a CSV file, concatenates all the fields in each row into a single string, sends the string to the OpenAI API to create an embedding vector, and then writes the embedding vector to a separate output file.

## Prerequisites

- Rust: The latest stable version of Rust is needed to compile and run this script. You can download it from the official [Rust website](https://www.rust-lang.org/).

- An OpenAI API key: You will need an OpenAI API key to use the OpenAI API. If you don't have one, you can obtain it from the [OpenAI website](https://www.openai.com/).

## Usage

1. Clone the repository:

```
git clone https://github.com/sudoQuigley/csv_to_embedding_vectors.git
```

2. Change into the cloned directory:

```
cd csv_to_embedding_vectors
```

3. Run the script with the path to your CSV file as an argument:

```
cargo run --release /path/to/your/file.csv
```

Replace `/path/to/your/file.csv` with the path to the CSV file you want to process.

4. The output files will be created in the current directory with the name `embedding_vector_<index>.txt`, where `<index>` is the index of the row in the CSV file.

## Notes

- The CSV file can have any number of columns. The script concatenates all the fields in each row into a single string to create the embedding vector.

- The script skips empty rows in the CSV file.

- The script expects the CSV file to have a header row.

- The OpenAI API key should be set in the environment variable `OPENAI_API_KEY`.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
