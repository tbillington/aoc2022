#![no_std]
#![no_main]

use core::str::FromStr;

extern crate libc;

fn elf_calories<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.split("\n\n").map(|l| {
        l.lines()
            .map(u32::from_str)
            .map(Result::unwrap_or_default)
            .sum::<u32>()
    })
}

fn part1(input: &str) -> u32 {
    elf_calories(input).max().unwrap_or_default()
}

fn part2(input: &str) -> u32 {
    let mut elfs = [0u32; 500];
    let mut elf_count = 0;

    for (i, calories) in elf_calories(input).enumerate() {
        elfs[i] = calories;
        elf_count = i;
    }

    let elfs = &mut elfs[..elf_count];

    elfs.sort_unstable_by(|a, b| b.cmp(a));

    elfs.iter().take(3).sum()
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let input = include_str!("day1.txt");

    libc::printf("part 1: %d\n\0".as_ptr() as *const i8, part1(input));
    libc::printf("part 2: %d\n\0".as_ptr() as *const i8, part2(input));

    0
}

#[cfg(not(test))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
