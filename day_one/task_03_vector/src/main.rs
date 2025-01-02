fn reverse_vector<T>(vec: &mut Vec<T>) {
    let mut left = 0;
    let mut right = vec.len().checked_sub(1).unwrap_or(0); // Use checked_sub to prevent overflow

    while left < right {
        vec.swap(left, right);
        left = left.checked_add(1).unwrap_or(usize::MAX); // Using checked_add
        right = right.checked_sub(1).unwrap_or(0); // Use checked_sub again for decrementing right
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    reverse_vector(&mut numbers);
    println!("Reversed: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::reverse_vector;

    #[test]
    fn test_reverse_vector_happy_cases() {
        // Test with an even number of elements
        let mut vec = vec![1, 2, 3, 4];
        reverse_vector(&mut vec);
        assert_eq!(vec, vec![4, 3, 2, 1]);

        // Test with an odd number of elements
        let mut vec = vec![1, 2, 3];
        reverse_vector(&mut vec);
        assert_eq!(vec, vec![3, 2, 1]);

        // Test with one element (no change expected)
        let mut vec = vec![1];
        reverse_vector(&mut vec);
        assert_eq!(vec, vec![1]);

        // Test with empty vector (no change expected)
        let mut vec = Vec::<i32>::new();
        reverse_vector(&mut vec);
        assert_eq!(vec, Vec::<i32>::new());
    }

    #[test]
    fn test_reverse_vector_unhappy_cases() {
        // No specific unhappy cases for this operation since it's in-place and doesn't rely on external conditions, 
        // but we can test for robustness:

        // Test with different types (e.g., characters)
        let mut vec = vec!['a', 'b', 'c', 'd'];
        reverse_vector(&mut vec);
        assert_eq!(vec, vec!['d', 'c', 'b', 'a']);

        // Test with a large vector (to ensure performance doesn't degrade unexpectedly)
        let mut large_vec = (0..1000).collect::<Vec<_>>();
        let expected = (0..1000).rev().collect::<Vec<_>>();
        reverse_vector(&mut large_vec);
        assert_eq!(large_vec, expected);
    }
}