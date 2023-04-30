use std::io;
use std::time::Instant;

mod sortHugeFile;

#[tokio::main]
async fn main() -> io::Result<()> {
    let input = "input.txt";
    let output_prefix = "sorted";
    let output = "output.txt";

    println!("Splitting file...");
    let start = Instant::now();
    let chunk_count = sortFile::split_file(input, output_prefix).await?;
    let elapsed = start.elapsed();
    println!("File split into {} chunks. Time elapsed: {:?}", chunk_count, elapsed);

    println!("Sorting chunks...");
    let start = Instant::now();
    sortFile::sort_chunks(chunk_count, output_prefix)?;
    let elapsed = start.elapsed();
    println!("Chunks sorted. Time elapsed: {:?}", elapsed);

    println!("Merging chunks...");
    let start = Instant::now();
    sortFile::merge_chunks(chunk_count, output_prefix, output).await?;
    let elapsed = start.elapsed();
    println!("Chunks merged. Time elapsed: {:?}", elapsed);

    println!("Sorting complete.");

    Ok(())
}
