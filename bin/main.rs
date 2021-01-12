use std::vec;

fn sieve_of_eratosthenes(n :usize) {
    /*
    Create a boolean array "prime[0..n]" and initialize
    all entries it as true. A value in prime[i] will
    finally be false )if i is Not a prime, else true.
    */
    let mut prime = vec![1];

    for _e in 0..n {
        prime.push(1);
    }
    let mut i = 2;

    while i*i<=n {
        // If prime[p] is not changed, then it is a prime
        if prime[i] == 1 {
            /*
            Update all multiples of p greater than or
            equal to the square of it
            numbers which are multiple of p and are
            less than p^2 are already been marked.
            */
            let mut j = i*i;
            while j <=n {
                prime[j] = 0;
                j+=i;
            }
        }
        i+=1;
    }

    print!("[OUTPUT] ");
    // Print all prime numbers
    for (k, v) in prime.iter().enumerate() {
       if *v == 1 && k >= 2 {
          print!("{} ", k);
       }
    }
    println!("");
}

// Driver Program to test above function
fn main() {
    let input:usize = 10;
    println!("[INPUT] {}", input);
    sieve_of_eratosthenes(input);
}
