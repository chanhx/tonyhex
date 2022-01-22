use std::{
    cmp::min,
    fs::File,
    io::{prelude::*, BufRead, BufReader, SeekFrom},
};

use anyhow::Result;

use crate::types::DisplayChar;

fn line(
    offset: u64,
    offset_width: usize,
    hexes: Vec<String>,
    chars: String,
    plain: bool,
) -> String {
    let hexes_len = hexes.len();
    if hexes_len > 8 {
        let width1 = 8 * 2 + (8 - 1) + if plain { 0 } else { (hexes_len - 8) * 5 };

        format!(
            "{:0width0$x}  {}  {:width1$}  {}{}\n",
            offset,
            hexes[0..8].join(" "),
            hexes[8..].join(" "),
            chars,
            if plain { "" } else { "\x1b[0m" },
            width0 = offset_width,
            width1 = width1,
        )
    } else {
        let width1 = 16 * 2 + (16 - 1) + 2 + if plain { 0 } else { hexes_len * 5 };

        format!(
            "{:0width0$x}  {:width1$} {}{}\n",
            offset,
            hexes.join(" "),
            chars,
            if plain { "" } else { "\x1b[0m" },
            width0 = offset_width,
            width1 = width1,
        )
    }
}

fn offset_bits_count(mut offset: u64) -> usize {
    let mut count = 8;
    offset >>= 32;

    while offset != 0 {
        count += 1;
        offset >>= 4;
    }

    count
}

fn chunk_to_hexes_chars(chunk: &[u8], plain: bool) -> (Vec<String>, String) {
    chunk
        .into_iter()
        .map(|b| {
            let b = *b;

            let ch = DisplayChar::new(b);
            let hex = format!("{:02x}", b);

            let c = ch.to_string();

            if plain {
                (hex, c)
            } else {
                let color = ch.color();
                (
                    format!("\x1b[{}m{}", color, hex),
                    format!("\x1b[{}m{}", color, c),
                )
            }
        })
        .unzip()
}

pub fn run(
    filename: &str,
    filesize: u64,
    offset: u64,
    length: Option<u64>,
    plain: bool,
) -> Result<()> {
    let mut file = match File::open(filename) {
        Err(err) => panic!("couldn't open {}: {}", filename, err),
        Ok(file) => file,
    };

    let mut offset = if offset > 0 {
        file.seek(SeekFrom::Start(offset)).unwrap()
    } else {
        0
    };

    let cap: usize = 1024 * 128;
    let (cap, max_offset) = match length {
        Some(len) => (min(cap, len as usize), offset + len),
        None => (cap, filesize),
    };

    let offset_width = offset_bits_count(max_offset);
    let mut reader = BufReader::with_capacity(cap, file);

    loop {
        let read_length = {
            let buffer = reader.fill_buf()?;

            let lines = buffer
                .chunks(16)
                .enumerate()
                .map(|(i, c)| {
                    let (hexes, chars) = chunk_to_hexes_chars(c, plain);
                    line(offset + (i << 4) as u64, offset_width, hexes, chars, plain)
                })
                .collect::<String>();

            println!("{}", lines);

            buffer.len()
        };

        offset += read_length as u64;
        if read_length == 0 || offset >= max_offset {
            break;
        }
    }

    Ok(())
}
