use amlich::{Calendar, GregorianMonth};
mod table;

fn main() {
    //    eprintln!("{:?}", GregorianMonth::new(2019, 9).get_bound());
    table::CalTable::new().run();
}
