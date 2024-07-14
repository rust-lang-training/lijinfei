mod util;
mod utils;

use utils::test_mod as t;
fn main() {
    util::test();
    t();
    utils::Area::test();

    let copy_file = "./20240710.zip";
    let target_file = "./20240710_copy.zip";

    let result = util::copy_file(copy_file, target_file);
}
