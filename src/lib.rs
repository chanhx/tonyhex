use std::{
    cmp::min,
    fs::File,
    io::{prelude::*, SeekFrom},
};

fn line(
    offset: u64,
    offset_width: usize,
    buf: [u8; 16], 
    n: usize,
) -> String {

    let hexes = buf.iter().enumerate().take(n).map(|(i, b)| {
        if i == 7 {
            format!("{:02x} ", b)
        } else {
            format!("{:02x}", b)
        }
    }).collect::<Vec<String>>().join(" ");

    let chars = buf.iter().take(n).map(|&b| {
        match b {
            0x21 ..= 0x7e => b as char,
            _ => '.',
        }
    }).collect::<String>();

    format!("{:0width$x}  {:48}  {:16}", offset, hexes, chars, width=offset_width)
}

fn offset_bits_count(mut offset: u64) -> usize {

    let mut count = 8;
    offset >>= 32;

    while offset != 0 {
        count += 1;
        offset >>= 4;
    }

    return count;
}

pub fn run(
    file: &mut File,
    filesize: u64,
    initial_offset: u64,
    length: Option<u64>,
) {
    let max_offset = match length {
        Some(len) => initial_offset + len,
        None => filesize,
    };

    let offset_width = offset_bits_count(max_offset);

    let mut buf = [0u8; 16];
    let mut offset = if initial_offset > 0 {
        file.seek(SeekFrom::Start(initial_offset)).unwrap()
    } else {0};

    loop {
        let n = min(
            file.read(&mut buf).expect("error reading file") as u64,
            max_offset - offset,
        );

        println!("{}", line(offset, offset_width, buf, n as usize));

        if n < 16 {break;}
        offset += n;
    }
}
