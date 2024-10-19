pub fn merge_sort<T: Ord + 'static + Copy + Default>(array: Vec<T>) -> Vec<T> {
    let middle = (array.len() as f64 / 2.0).ceil() as usize;

    let result = _merge_sort();

    return array;
}

fn _merge_sort<T: Ord + 'static + Copy + Default>(array: Vec<T>, middle: usize) {

}

fn merge<T: Ord + 'static + Copy + Default>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    let first_len = first.len();
    let second_len = second.len();
    let final_len = first_len + second_len;
    let mut merged_vec: Vec<T> = vec![T::default(); final_len];

    let mut idx_first: usize = 0;
    let mut idx_second: usize = 0;
    let mut idx_final: usize = 0;

    while idx_final < final_len {
        if idx_first < first_len && idx_second < second_len {
            let first_elem = first[idx_first];
            let second_elem = second[idx_second];

            if first_elem <= second_elem {
                merged_vec[idx_final] = first_elem;
                idx_first += 1;
            } else {
                merged_vec[idx_final] = second_elem;
                idx_second += 1;
            }

            idx_final += 1;
        }

        if idx_first >= first_len && idx_second < second_len {
            while idx_second < second_len {
                merged_vec[idx_final] = second[idx_second];
                idx_final += 1;
                idx_second += 1;
            }
        }

        if idx_second >= second_len && idx_first < first_len {
            while idx_first < first_len {
                merged_vec[idx_final] = first[idx_first];
                idx_final += 1;
                idx_first += 1;
            }
        }
    }

    return merged_vec;
}

#[cfg(test)]
mod test {
    use log::debug;
    use super::*;

    #[test]
    fn test_merge() {
        let a1 = vec![2, 4, 5, 7];
        let a2 = vec![1, 2, 3, 6];
        let expected = vec![1, 2, 2, 3, 4, 5, 6, 7];

        let actual = merge(a1, a2);

        debug!("{:?}",actual);
        assert_eq!(expected[0], actual[0]);
        assert_eq!(expected[1], actual[1]);
        assert_eq!(expected[2], actual[2]);
        assert_eq!(expected[3], actual[3]);
        assert_eq!(expected[4], actual[4]);
        assert_eq!(expected[5], actual[5]);
        assert_eq!(expected[6], actual[6]);
        assert_eq!(expected[7], actual[7]);
    }

    #[test]
    fn test_merge_sort1() {
        let array = vec![2, 4, 5, 7, 1, 2, 3, 6];
        let expected = vec![1, 2, 2, 3, 4, 5, 6, 7];

        let actual = merge_sort(array);

        assert_eq!(expected[0], actual[0]);
        assert_eq!(expected[1], actual[1]);
        assert_eq!(expected[2], actual[2]);
        assert_eq!(expected[3], actual[3]);
        assert_eq!(expected[4], actual[4]);
        assert_eq!(expected[5], actual[5]);
        assert_eq!(expected[6], actual[6]);
        assert_eq!(expected[7], actual[7]);
    }

    #[test]
    fn test_merge_sort2() {
        let array = vec![2, 4, 5, 7, 1, 3, 6];
        let expected = vec![1, 2, 3, 4, 5, 6, 7];

        let actual = merge_sort(array);

        assert_eq!(expected[0], actual[0]);
        assert_eq!(expected[1], actual[1]);
        assert_eq!(expected[2], actual[2]);
        assert_eq!(expected[3], actual[3]);
        assert_eq!(expected[4], actual[4]);
        assert_eq!(expected[5], actual[5]);
        assert_eq!(expected[6], actual[6]);
    }
}