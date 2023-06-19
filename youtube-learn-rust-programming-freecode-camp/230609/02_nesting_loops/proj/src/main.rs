// https://youtu.be/BpPEoZW5IiY?t=16868

fn main() {
    let mut count = 0;

    // use labeled loops, those are useful when we want to break from an outer loop
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This will break only the inner loop
                break 'inner1;
            }
            count += 2
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This will break the outer loop
                break 'outer;
            }

            // This will continue escape and continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!")
}
