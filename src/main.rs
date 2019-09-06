use amlich::Calendar;
mod table;

fn main() {
    let tbl = table::Table::of(amlich::GregorianMonth::new(2019, 9));
    eprintln!("{}", tbl);
}
