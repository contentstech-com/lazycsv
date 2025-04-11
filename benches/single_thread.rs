use std::{hint::black_box, io::Cursor};

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use lazycsv::{Csv, CsvIterItem};
use rand::{Rng, SeedableRng as _};

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const QUOTED_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\",";
const ROWS: usize = 100_000;
const COLS: usize = 30;
const MIN_CHARS: usize = 3;
const MAX_CHARS: usize = 100;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Quoting {
    Never,
    Always,
    Random,
}

fn gen_random_str<T: Rng>(rng: &mut T, quoting: Quoting) -> String {
    let should_quote =
        quoting == Quoting::Always || (quoting == Quoting::Random && rng.gen_bool(0.5));
    let content: String = (0..rng.gen_range(MIN_CHARS..MAX_CHARS))
        .map(|_| {
            if should_quote {
                QUOTED_CHARS[rng.gen_range(0..QUOTED_CHARS.len())] as char
            } else {
                CHARS[rng.gen_range(0..CHARS.len())] as char
            }
        })
        .collect();

    if should_quote {
        format!("\"{}\"", content.replace("\"", "\"\""))
    } else {
        content
    }
}

fn prepare() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut buf_never = Vec::with_capacity(ROWS * COLS * ((MAX_CHARS - MIN_CHARS) / 2 + MIN_CHARS));
    let mut buf_always =
        Vec::with_capacity(ROWS * COLS * ((MAX_CHARS - MIN_CHARS) / 2 + MIN_CHARS));
    let mut buf_random =
        Vec::with_capacity(ROWS * COLS * ((MAX_CHARS - MIN_CHARS) / 2 + MIN_CHARS));

    let mut rng = rand::rngs::StdRng::from_seed(b"f3a90c67b3ca86afd62658c1b30f1f12".to_owned());
    for (buf, quoting) in [
        (&mut buf_never, Quoting::Never),
        (&mut buf_always, Quoting::Always),
        (&mut buf_random, Quoting::Random),
    ] {
        for _ in 0..ROWS {
            for col in 0..COLS {
                buf.extend_from_slice(gen_random_str(&mut rng, quoting).as_bytes());
                if col != 29 {
                    buf.push(b',');
                }
            }
            buf.push(b'\n');
        }
    }

    (buf_never, buf_always, buf_random)
}

pub fn lazy_csv(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for item in Csv::new(slice) {
            if let CsvIterItem::Cell(cell) = item {
                black_box(cell.try_as_str().unwrap());
            }
        }
    })
}

pub fn lazy_csv_into_rows(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for row in Csv::new(slice).into_rows::<COLS>() {
            for cell in row.unwrap() {
                black_box(cell.try_as_str().unwrap());
            }
        }
    })
}

pub fn lazy_csv_raw(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for cell in Csv::new(slice) {
            black_box(cell);
        }
    })
}

pub fn lazy_csv_into_rows_raw(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for row in Csv::new(slice).into_rows::<COLS>() {
            for cell in row.unwrap() {
                black_box(cell);
            }
        }
    })
}

pub fn csv(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        let cursor = Cursor::new(slice);
        for row in csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(cursor)
            .into_records()
        {
            for cell in row.unwrap().into_iter() {
                black_box(cell);
            }
        }
    })
}

fn bench_parsers(c: &mut Criterion) {
    let (buf_never, buf_always, buf_random) = prepare();

    for (buf, title) in [
        (&buf_never, "No quotes"),
        (&buf_always, "Always quoted"),
        (&buf_random, "Randomly quoted"),
    ] {
        let mut group = c.benchmark_group(title);
        group.sample_size(50);
        group.bench_with_input("lazy_csv", &buf.clone(), |b, buf| lazy_csv(b, buf));
        group.bench_with_input("lazy_csv (into_rows)", &buf.clone(), |b, buf| {
            lazy_csv_into_rows(b, buf)
        });
        group.bench_with_input("lazy_csv (raw)", &buf.clone(), |b, buf| {
            lazy_csv_raw(b, buf)
        });
        group.bench_with_input("lazy_csv (into_rows, raw)", &buf.clone(), |b, buf| {
            lazy_csv_into_rows_raw(b, buf)
        });
        group.bench_with_input("csv", &buf.clone(), |b, buf| csv(b, buf));
        group.finish();
    }
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
