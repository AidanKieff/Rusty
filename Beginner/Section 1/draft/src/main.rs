use num::integer::sqrt;

// There's a prime number hiding in our array of integers!
// The function below tries to find the prime number by checking each element,
// and finding its divisor. If none is found, then it's a prime number and
// its search ends!

// But it seems that its search never does end, when there's clearly a
// prime number there. Fix the function so that it returns the prime number.

fn main() {
    let numbers = 9;
    let prime = get_prime(numbers);
}

fn get_prime( x: i32 ) -> i32 {

    // Loop through every element in the array
    
    'outer: loop {

        // Find a divisor.
        let mut n = 2;
        'inner: loop {
            
            // If a number can be evenly divided by another number except 1 and itself,
            // then it's not a prime.
            // Break out here to move on to the next element.
            if x % n == 0 {
                if x == 2 {
                    break 'outer;
                }
                
                break;
            }

            // If no divisors are found, then we've found a prime!
            // Break out of the loop here.
            if n >= sqrt(x) {
                break 'outer;
                
            }
            
            // Otherwise, move to the next element.
            n += 1;
        }
    }
    println!("The first prime number in the array is {}.", x);
    x
}