use logic::id::DimensionRange::{Any, LimitRange, Single};
use logic::id::SpaceTimeId;

fn main() {
    let stid = SpaceTimeId::new(3, LimitRange(4, 3), Single(3), Single(3), 0, Any).unwrap();

    println!("{}", stid);
}
