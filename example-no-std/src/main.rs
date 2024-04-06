#![no_main]
#![no_std]

use core::panic::PanicInfo;

use libc_print::std_name::println;
use tsv_reader::err::Error;
use tsv_reader::reader::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[derive(Debug, Read)]
struct Alice<'a> {
    eat_me: u8,
    who_am_i: &'a str,
}

#[derive(Debug, Read)]
enum Bob {
    Cat,
    Dog,
    Fox(i32),
    Mouse { head: u32, tail: [u8; 4] },
}

fn parse_data(input: &[u8]) -> Result<(Alice<'_>, impl Iterator<Item = Bob> + '_), Error> {
    let mut lines = core::str::from_utf8(input)?.split('\n');
    let alice = lines.next().ok_or(Error)?;
    Ok((
        Walker::parse_one_line(alice)?,
        lines.map_while(|line| Walker::parse_one_line(line).ok()),
    ))
}

const DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.tsv"));

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let (alice, bobs) = parse_data(DATA).unwrap();
    println!("{alice:?}");
    for bob in bobs {
        println!("{bob:?}");
    }
    0
}
