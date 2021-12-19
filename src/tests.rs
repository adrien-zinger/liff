extern crate test;
use test::Bencher;

use super::*;
use std::fs;

lazy_static::lazy_static! {
  static ref FROM: String = uuid_readable_rs::generate();
  static ref TO: String = uuid_readable_rs::generate();
}

macro_rules! utf8 {
    ($e:expr) => {
        std::str::from_utf8($e).unwrap()
    };
}

#[test]
fn run_tests() {
    // Binaries
    let from_to = format!("\nFrom: {}\nTo: {}", FROM.to_owned(), TO.to_owned());
    let from: Vec<u8> = FROM.as_bytes().into();
    let to: Vec<u8> = TO.as_bytes().into();
    let diff = diff::diff::<u8>(&from, &to);
    diffio::write(std::path::Path::new("diff.d"), diff.clone());
    diffio::debug_u8_to_char(&diff);
    let diff = diffio::read(std::path::Path::new("diff.d"));
    let res = apply::apply(from, &diff);
    fs::write("dest.pack", res.clone()).unwrap();
    assert_eq!(utf8!(&res), utf8!(&to), "{}", from_to);
}

#[bench]
fn benchmark_binary(b: &mut Bencher) {
    let from = "first.pack";
    let to = "second.pack";
    let first = fs::read(from).unwrap();
    let second = fs::read(to).unwrap();
    b.iter(move || {
        diff::levenstein(&first, &second);
    });
}

#[bench]
fn benchmark_text(b: &mut Bencher) {
    let from: Vec<u8> = FROM.as_bytes().into();
    let to: Vec<u8> = TO.as_bytes().into();
    b.iter(move || {
        diff::levenstein(&from, &to);
    });
}

#[bench]
fn dev_test(b: &mut Bencher) {
    let from: Vec<u8> = FROM.as_bytes().into();
    let to: Vec<u8> = TO.as_bytes().into();
    let mut res = 0;
    let chunks_from = from.chunks_exact(1);
    println!("size {}", chunks_from.len());
    b.iter(|| {
        let chunks_from = from.chunks_exact(1);
        let chunks_to = to.chunks_exact(1);
        res = 0;
        chunks_from.zip(chunks_to.into_iter()).for_each(|(a, b)| {
            if a == b {
                res += 1;
            }
        });
    });
    println!("equals {}", res);
}

#[bench]
fn benchmark_text_external(b: &mut Bencher) {
    let from = FROM.as_bytes();
    let to = TO.as_bytes();
    b.iter(|| {
        let (_, matrix) = levenshtein_diff::distance(from, to);
        let _ = levenshtein_diff::generate_edits(from, to, &matrix);
    });
}

#[bench]
fn benchmark_binary_external(b: &mut Bencher) {
    let first = "first.pack";
    let second = "second.pack";
    let from = fs::read(first).unwrap();
    let to = fs::read(second).unwrap();
    b.iter(|| {
        let (_, matrix) = levenshtein_diff::distance(&from, &to);
        let _ = levenshtein_diff::generate_edits(&from, &to, &matrix);
    });
}
