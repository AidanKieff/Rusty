use numm::integer::sqrt;
fn main() {
    let numbers = [36, 25, 49, 3, 64, 16, 9];
    let prime = get_prime(numbers);
}

fn get_prime(arr: [i32; 7]) -> i32 {

    // loop through entire array
    let mut i = 0;
    'outer: loop{

        // find a divisor
        let mut n = 2;
        'inner: loop {

            // If a number can be evenly divided by another number except 1 and itself,
            // then it's not a prime.
            // Break out of here to move to the next element
            if arr[i] % n == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i += 1;
                break;
            }
        }
    }
    println!("The first prime number in the array is {}.", arr[i]);
    arr[i]
}