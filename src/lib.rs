const MIN_NUMBER: i64 = -3000;
const MAX_NUMBER: i64 = 3000;
const NUMBER_RANGE: i64 = MAX_NUMBER - MIN_NUMBER;


/// Educated approach to the three sums problem involving non-comparison based sorting and a simplified HashSet
/// 
/// # Runtime Complexity
/// ## Best
/// O(N)
/// ## Average
/// O(N^2)
/// ## Worst
/// O(N^2)
pub fn three_sum(source: &[i64]) -> Vec<[i64; 3]> {
    // Counting sort since we know the range of numbers
    let mut sorting_array = [0usize; NUMBER_RANGE as usize];
    for n in source {
        let i = (n - MIN_NUMBER) as usize;
        sorting_array[i] += 1;
    }

    // Produce a sorted, deduplicated array of numbers (and their count) from the source array
    // in O(n) time. Unique array is of the form [(num_1, count_1), (num_2, count_2), ... ]
    let mut unique_array = Vec::new();
    for (i, count) in sorting_array.into_iter().enumerate() {
        let n = i as i64 + MIN_NUMBER;
        if count > 0 {
            unique_array.push((n, count));
        }
    }

    let mut result = Vec::new();

    // Iterate from the beginning of the unique array to the end
    for left in 0..unique_array.len() {
        let left_num = unique_array[left].0;
        let left_num_count = unique_array[left].1;

        // Iterate from the end of the unique array to left inclusive
        for right in (left..unique_array.len()).rev() {
            let right_num = unique_array[right].0;
            let right_num_count = unique_array[right].1;

            // While there is no guarantee that an appropriate third number exists,
            // we are guaranteed that it must lie between the left and right number
            let third_num = 0 - left_num - right_num;

            // The two following range checks prevent duplicate solutions
            // as the algorithm will only find solutions within the current range

            // Since we are iterating in reverse, we know that if we reach
            // this condition that there are no more solutions for the given
            // left number
            if third_num > right_num {
                break;
            }

            // There may still be solutions, so we must still iterate
            if third_num < left_num {
                continue;
            }

            // Early exit cases with duplicate number
            // 
            // If the third number is equal to the left or right number, then
            // we do not need to search for the third number as its presence is confirmed.
            // Instead, we just need to check if there are enough numbers to produce
            // a solution
            if third_num == left_num {
                if third_num == right_num {
                    if left_num_count >= 3 {
                        // The only case where all three are equal is when they are all zero
                        // However, there must be at least 3 zeros for this to work
                        result.push([0, 0, 0]);
                    }
                } else if left_num_count >= 2 {
                    // There is enough of the left number to be a solution
                    // If there is only 1 of the left number, then it cannot appear in
                    // two places
                    result.push([left_num, left_num, right_num]);
                }

                continue;

            } else if third_num == right_num {
                if right_num_count >= 2 {
                    // See case for the left number
                    result.push([left_num, right_num, right_num]);
                }
                continue;
            }

            // Use the sorting array as a hashset, where the hash is the third number
            // Remember, the sorting array stores the counts of each number,
            // so a count of more than 0 means that the number exists
            let third_index = (third_num - MIN_NUMBER) as usize;
            if sorting_array[third_index] > 0 {
                result.push([left_num, third_num, right_num]);
            }
        }
    }

    return result;
}

/// Iterates through every possible combination of 3 numbers for a solution
/// 
/// # Runtime Complexity
/// O(n^3)
pub fn naive_three_sum(source: &[i64]) -> Vec<[i64; 3]> {
    let mut result = Vec::new();

    for i in 0..(source.len() - 2) {
        for j in (i + 1)..(source.len() - 1) {
            for k in (j + 1)..source.len() {
                if source[i] + source[j] + source[k] == 0 {
                    let mut solution = [source[i], source[j], source[k]];
                    solution.sort_unstable();
                    result.push(solution);
                }
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn it_works() {
        fastrand::seed(1774478);

        for i in 0..100 {
            let mut src = Vec::with_capacity(fastrand::usize(250..500));
            for _i in 0..src.capacity() {
                src.push(fastrand::i64(MIN_NUMBER..MAX_NUMBER));
            }

            let naive_soln = naive_three_sum(&src).into_iter().collect::<HashSet<_>>();
            let maybe_soln = three_sum(&src).into_iter().collect::<HashSet<_>>();

            assert_eq!(naive_soln, maybe_soln);
            println!("{i:<5} correct");
        }
    }
}
