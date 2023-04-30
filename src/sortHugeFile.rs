use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
use tokio::fs::File as TokioFile;
use tokio::io::BufReader as TokioBufReader;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufWriter;
use tokio::io::AsyncWriteExt;
use tokio::fs::File as TokioFile;

const CHUNK_SIZE: usize = 100 * 1024 * 1024; // 100MB

async fn split_file(input: &str, output_prefix: &str) -> std::io::Result<usize> {
    let file = TokioFile::open(input).await?;
    let reader = TokioBufReader::new(file);
    let mut chunk = Vec::with_capacity(CHUNK_SIZE);
    let mut chunk_counter = 0;
    let mut lines_stream = reader.lines();

    while let Some(line) = lines_stream.next_line().await? {
        chunk.push(line);

        if chunk.len() == CHUNK_SIZE {
            let chunk_path = format!("{}_chunk_{}.txt", output_prefix, chunk_counter);
            write_chunk(&chunk, &chunk_path)?;
            chunk_counter += 1;
            chunk.clear();
        }
    }

    if !chunk.is_empty() {
        let chunk_path = format!("{}_chunk_{}.txt", output_prefix, chunk_counter);
        write_chunk(&chunk, &chunk_path)?;
    }

    Ok(chunk_counter)
}

fn sort_chunks(chunk_count: usize, output_prefix: &str) -> std::io::Result<()> {
    (0..=chunk_count).into_par_iter().try_for_each(|i| {
        let chunk_path = format!("{}_chunk_{}.txt", output_prefix, i);
        let mut chunk = read_chunk(&chunk_path)?;
        chunk.par_sort();
        write_chunk(&chunk, &chunk_path)?;
        Ok(())
    })?;

    Ok(())
}


async fn merge_chunks(chunk_count: usize, output_prefix: &str, output: &str) -> std::io::Result<()> {
    let mut heap = BinaryHeap::new();
    let mut chunk_files = Vec::new();

    for i in 0..=chunk_count {
        let chunk_path = format!("{}_chunk_{}.txt", output_prefix, i);
        let file = TokioFile::open(&chunk_path).await?;
        let reader = TokioBufReader::new(file);
        let mut lines = reader.lines();

        if let Some(line) = lines.next_line().await? {
            heap.push(Reverse(MergeElement {
                value: line.parse::<f64>().unwrap(),
                chunk_index: i,
            }));
            chunk_files.push(lines);
        }
    }

    let output_file = TokioFile::create(output).await?;
    let mut writer = TokioBufWriter::new(output_file);

    while let Some(Reverse(top)) = heap.pop() {
        writeln!(writer, "{}", top.value).await?;

        if let Some(line) = chunk_files[top.chunk_index].next_line().await? {
            heap.push(Reverse(MergeElement {
                value: line.parse::<f64>().unwrap(),
                chunk_index: top.chunk_index,
            }));
        }
    }

    writer.flush().await?;

    Ok(())
}
