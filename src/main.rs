use crate::module1_overview::module_1;
use crate::module2_basics::module_2;
use crate::module3_types::module_3;
use crate::module4_logic::module_4;
mod module1_overview;
mod module2_basics;
mod module3_types;
mod module4_logic;

fn main() {
    module_1();
    module_2();
    module_3();
    module_4();
}
