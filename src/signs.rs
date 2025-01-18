use rand::Rng;
pub enum SymblesChoice {
    OnlyNumbers,
    AllShift,
    All,
}

use SymblesChoice::{All, AllShift, OnlyNumbers};

pub static SYMBLES_IN_NUMBERS: [char; 10] = [')', '!', '@', '#', '$', '%', '^', '&', '*', '('];
pub static EXTRA_SYMBLES_WITH_SHIFT: [char; 10] =
    ['~', '_', '+', '{', '}', ':', '\'', '<', '>', '?'];
pub static EXTRA_SYMBLES_WITHOUT_SHIFT: [char; 10] =
    ['`', '-', '=', '[', ']', ';', '\'', ',', '.', '/'];

pub fn get_rand_symble(choice: &SymblesChoice) -> &'static char {
    let mut rng = rand::thread_rng();
    let index_in_section: usize = rng.gen_range(0..10); // 生成 0 到 9 之间的随机索引

    match choice {
        OnlyNumbers => &SYMBLES_IN_NUMBERS[index_in_section],
        AllShift => {
            let which_section = rng.gen_range(0..2);
            match which_section {
                0 => &SYMBLES_IN_NUMBERS[index_in_section],
                1 => &EXTRA_SYMBLES_WITH_SHIFT[index_in_section],
                _ => unreachable!("程序内部出错!"),
            }
        }
        All => {
            let which_section = rng.gen_range(0..3);
            match which_section {
                0 => &SYMBLES_IN_NUMBERS[index_in_section],
                1 => &EXTRA_SYMBLES_WITH_SHIFT[index_in_section],
                2 => &EXTRA_SYMBLES_WITHOUT_SHIFT[index_in_section],
                _ => unreachable!("程序内部出错!"),
            }
        }
    }
}
