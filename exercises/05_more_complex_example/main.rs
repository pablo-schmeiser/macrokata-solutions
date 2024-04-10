////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! for_2d {
    ($name1:ident <$type1:ty> in $range1:expr, $name2:ident <$type2:ty> in $range2:expr, $code:block) => {
        for $name1 in $range1 {
            let $name1: $type1 = $name1;
            for $name2 in $range2 {
                let $name2: $type2 = $name2;
                $code
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
