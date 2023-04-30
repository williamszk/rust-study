#[allow(dead_code)]
#[allow(unused_variables)]

const EUROPE: u8 = 0;
const ASIA: u8 = 1;
const AFRICA: u8 = 2;
const AMERICA: u8 = 3;
const OCEANIA: u8 = 4;

enum Continent {
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}

fn main() {
    let continent = ASIA;
    if continent == EUROPE {
        println!("E");
    } else if continent == ASIA {
        println!("As");
    } else if continent == AFRICA {
        println!("Af");
    } else if continent == AMERICA {
        println!("Am");
    } else if continent == OCEANIA {
        println!("O");
    }

    // the better way to do the above
    // using enums and match statements
    let contin = Continent::Asia;

    match contin {
        Continent::Europe => println!("E"),
        Continent::Asia => println!("As"),
        Continent::Africa => println!("Af"),
        Continent::America => println!("Am"),
        Continent::Oceania => println!("O"),
    }

    // ================================================================
    enum T {
        A,
        B,
        C,
        D,
    };

    let n: i32 = T::D;
    // let e: T = 1; // this will give an error
    // 1 is not of the T type
    // we can't implicitly convert enums to numbers or the other way around
}
