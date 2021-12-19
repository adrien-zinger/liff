use std::io::Write;
use std::vec::Vec;

type Diff<T> = Vec<(u8, u32, u32, Vec<T>, Vec<T>)>;

fn get_bytes_in(number: u32) -> Vec<u8> {
    let mut ret = Vec::new();
    for byte in number.to_be_bytes() {
        if byte > 0 {
            ret.push(byte);
        }
    }
    ret
}

pub fn write(file_path: &std::path::Path, diff: Diff<u8>) {
    let mut res = std::vec::Vec::new();
    for operation in diff.iter() {
        let mut description = operation.0 as u8;
        description <<= 3;
        let mut position_bytes = get_bytes_in(operation.1);
        description += position_bytes.len() as u8;
        description <<= 3;
        let mut raw_size_bytes = get_bytes_in(operation.3.len() as u32);
        description += raw_size_bytes.len() as u8;
        res.push(description);
        res.append(&mut position_bytes);
        res.append(&mut raw_size_bytes);
        if operation.0 == 1 {
            res.append(&mut operation.3.clone());
        }
        if operation.0 == 0 {
            let mut subraw_size_bytes = get_bytes_in(operation.3.len() as u32);
            res.push(subraw_size_bytes.len() as u8);
            res.append(&mut subraw_size_bytes);
            res.append(&mut operation.4.clone());
        }
    }
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(&snap::raw::Encoder::new().compress_vec(&res).unwrap())
        .unwrap();
}

pub fn write_char(file_path: &std::path::Path, diff: Diff<&str>) {
    let mut res = std::vec::Vec::new();
    for operation in diff.iter() {
        let mut description = operation.0 as u8;
        description <<= 3;
        let mut position_bytes = get_bytes_in(operation.1);
        description += position_bytes.len() as u8;
        description <<= 3;
        let mut raw_size_bytes = get_bytes_in(operation.3.len() as u32);
        description += raw_size_bytes.len() as u8;
        res.push(description);
        res.append(&mut position_bytes);
        res.append(&mut raw_size_bytes);
        if operation.0 == 1 {
            res.append(&mut operation.3.join("").as_bytes().to_vec());
        }
        if operation.0 == 0 {
            let mut subraw_size_bytes = get_bytes_in(operation.3.len() as u32);
            res.push(subraw_size_bytes.len() as u8);
            res.append(&mut subraw_size_bytes);
            res.append(&mut operation.4.join("").as_bytes().to_vec());
        }
    }
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(&snap::raw::Encoder::new().compress_vec(&res).unwrap())
        .unwrap();
}

pub fn read(file_path: &std::path::Path) -> Diff<u8> {
    let mut diff_file = snap::raw::Decoder::new()
        .decompress_vec(&std::fs::read(file_path).unwrap())
        .unwrap();
    diff_file.reverse();
    let mut diff = Vec::new();
    while !diff_file.is_empty() {
        let description = diff_file.pop().unwrap();
        let operation = (description & 0b1100_0000) >> 6;
        let position_bytes = (description & 0b0011_1000) >> 3;
        let raw_size_bytes = description & 0b0000_0111;
        let mut position = 0u32;
        for _ in 0..position_bytes {
            position <<= 8;
            position += diff_file.pop().unwrap() as u32;
        }
        let mut raw_size = 0u32;
        for _ in 0..raw_size_bytes {
            raw_size <<= 8;
            raw_size += diff_file.pop().unwrap() as u32;
        }
        let mut raw = Vec::new();
        if operation == 1 {
            for _ in 0..raw_size {
                raw.push(diff_file.pop().unwrap());
            }
        }
        let mut subraw = Vec::new();
        if operation == 0 {
            let subraw_size_bytes = diff_file.pop().unwrap();
            let mut subraw_size = 0u32;
            for _ in 0..subraw_size_bytes {
                subraw_size <<= 8;
                subraw_size += diff_file.pop().unwrap() as u32;
            }
            for _ in 0..subraw_size {
                subraw.push(diff_file.pop().unwrap());
            }
        }
        diff.push((operation, position, raw_size, raw, subraw));
    }
    diff
}

pub fn debug<T: std::fmt::Debug>(diff: &Diff<T>) {
    for operation in diff {
        let op_name = match operation.0 {
            0 => "substitution",
            1 => "add",
            _ => "delete",
        };
        println!("{}\n{:?}\n{:?}", op_name, operation.3, operation.4);
    }
}

pub fn debug_u8_to_char(diff: &Diff<u8>) {
    for operation in diff {
        let op_name = match operation.0 {
            0 => "substitution",
            1 => "add",
            _ => "delete",
        };
        let source: Vec<char> = operation.3.iter().map(|u| *u as char).collect();
        let target: Vec<char> = operation.4.iter().map(|u| *u as char).collect();
        println!("{}: {}\n{:?}\n{:?}", operation.1, op_name, source, target);
    }
}
