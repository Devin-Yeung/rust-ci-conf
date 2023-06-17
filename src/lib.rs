use rand::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    if rand::random() {
        // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
