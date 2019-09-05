use amlich::Calendar;

fn main() {
    let bound = amlich::GregorianMonth::new(2019, 9).get_bound();
    let iter = bound.iter();
    for x in iter {
        eprintln!("{:?} {:?} {}", x, x.to_lunar(), x.day_of_week());
    }
}
