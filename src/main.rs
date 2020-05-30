enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

impl Day {
    fn is_weekend(&self)->bool {
        match self {
            Day::Saturday | Day::Sunday => true,
            _=> false
        }
    }
}

fn main() {
    let d = Day::Sunday;
    println!("Is it a weekend: {}", d.is_weekend());
}