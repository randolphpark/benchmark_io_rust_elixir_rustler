#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{Env, Term, NifResult, Encoder};
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

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
      ("add", 2, add),
      ("return_string", 0, return_string),
      ("return_list", 0, return_list),
      ("read_file", 1, read_file)
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn return_string<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    Ok(format!("Hello world!").encode(env))
}

fn return_list<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    Ok(vec![1,2,3,4].encode(env))
}

fn read_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: u64 = try!(args[0].decode());

    let nums = vec![1,2,3,4,5,6,7,8,9,0,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8];

    let nums_iter = nums.iter();

    let mut f = match File::open("native/nifreader/database_fixture") {
        Ok(s) => s,
        Err(e) => panic!(e),
    };

    let mut buffer: [u8; 14] = [0; 14];

    for val in nums_iter {

      f.seek(SeekFrom::Start(*val));

      f.read_exact(&mut buffer);

      let result = buffer.clone();
    }

    f.seek(SeekFrom::Start(num1));

    f.read_exact(&mut buffer);

    let result = buffer.clone();


    Ok(result.encode(env))
    // Ok(num1.encode(env))
}
