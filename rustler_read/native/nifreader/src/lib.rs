#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate byteorder;

use rustler::{Env, Term, NifResult, Encoder};
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs;
use std::io::Cursor;

static FILE_PATH: &str = "native/nifreader/hibp_binary";
// native/nifreader/hibp_binary
static RECORD_SIZE: u64 = 14; // bytes
mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.NifReader",
    [
      ("seek_line", 1, seek_line),
      ("seek_29_times", 1, seek_29_times),
      ("pwn_check", 1, pwn_check),
    ],
    None
}

fn seek_line<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let index: u64 = try!(args[0].decode());

    let mut buffer: [u8; 14] = [0; 14];
    let mut f = File::open(FILE_PATH).expect("Fail to open the file");

    f.seek(SeekFrom::Start(index * RECORD_SIZE))
        .expect("Fail to seek record");
    f.read_exact(&mut buffer).expect("Fail to read buffer");

    let result = buffer.clone();

    Ok(result.encode(env))
}

fn seek_29_times<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let index: u64 = try!(args[0].decode());

    let nums = vec![1,2,3,4,5,6,7,8,9,0,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8];
    let nums_iter = nums.iter();

    let mut f = File::open(FILE_PATH).expect("Fail to open the file");
    let mut buffer: [u8; 14] = [0; 14];

    for val in nums_iter {
        f.seek(SeekFrom::Start((*val) * RECORD_SIZE))
            .expect("Fail to seek record");
        f.read_exact(&mut buffer).expect("Fail to read buffer");
        let _result = buffer.clone();
    }
    Ok(index.encode(env))
}

fn pwn_check<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let sha: Vec<u8> = try!(args[0].decode());
    let result = check(from_slice(&sha));
    Ok(result.encode(env))
}

fn from_slice(bytes: &[u8]) -> [u8; 10] {
    let mut array = [0; 10];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn check(target: [u8; 10]) -> u32 {
    let mut buffer: [u8; 14] = [0; 14];
    let mut f = File::open(FILE_PATH).expect("Fail to open the file");
    check_pwn(0, record_count(), target, &mut f, &mut buffer)
}

fn record_count() -> u64 {
    let metadata = fs::metadata(FILE_PATH).expect("Fail to read file size");
    metadata.len() / RECORD_SIZE
}

fn check_pwn(
    start: u64,
    end: u64,
    target: [u8; 10],
    f: &mut std::fs::File,
    buffer: &mut [u8; 14],
) -> u32 {
    if end - start == 0 {
        let record = read_line(f, buffer, start);
        if &target[0..10] == &record[0..10] {
            let mut rdr = Cursor::new(&record[10..]);
            rdr.read_u32::<BigEndian>().unwrap()
        } else {
            0
        }
    } else if end - start == 1 {
        let record = read_line(f, buffer, start);
        if &target[0..10] == &record[0..10] {
            let mut rdr = Cursor::new(&record[10..]);
            rdr.read_u32::<BigEndian>().unwrap()
        } else {
            check_pwn(start, start, target, f, buffer)
        }
    } else {
        let middle_index = start + (end - start) / 2;
        let record = read_line(f, buffer, middle_index);

        if &target[0..10] > &record[0..10] {
            check_pwn(middle_index, end, target, f, buffer)
        } else if &target[0..10] < &record[0..10] {
            check_pwn(start, middle_index, target, f, buffer)
        } else {
            let mut rdr = Cursor::new(&record[10..]);
            rdr.read_u32::<BigEndian>().unwrap()
        }
    }
}

pub fn read_line(f: &mut std::fs::File, buffer: &mut [u8; 14], line_num: u64) -> [u8; 14] {
    f.seek(SeekFrom::Start(line_num * RECORD_SIZE))
        .expect("Fail to seek record");
    f.read_exact(buffer).expect("Fail to read buffer");
    buffer.clone()
}

