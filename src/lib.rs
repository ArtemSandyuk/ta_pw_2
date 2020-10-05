use std::cmp::min;

#[allow(dead_code)]
fn merge_sort_iterative(a: &mut [isize]) {
    if a.len() == 0 {
        return;
    }

    let mut i = 1;
    while i < a.len() {
        for j in (0..a.len() - i).step_by(2 * i) {
            merge(a, j, j + i, min(j + 2 * i, a.len()));
            println!("i = {:?}, j = {:?}: {:?}", i, j, a);
        }
        i *= 2;
    }
}

fn merge(a: &mut [isize], left: usize, mid: usize, right: usize) {
    let mut it_1 = 0;
    let mut it_2 = 0;
    let mut result = vec![0; right - left];

    while ((left + it_1) < mid) && ((mid + it_2) < right) {
        if a[left + it_1] < a[mid + it_2] {
            result[it_1 + it_2] = a[left + it_1];
            it_1 += 1;
        } else {
            result[it_1 + it_2] = a[mid + it_2];
            it_2 += 1;
        }
    }

    while (left + it_1) < mid {
        result[it_1 + it_2] = a[left + it_1];
        it_1 += 1;
    }

    while (mid + it_2) < right {
        result[it_1 + it_2] = a[mid + it_2];
        it_2 += 1;
    }

    for i in 0..(it_1 + it_2) {
        a[left + i] = result[i];
    }
}

#[allow(dead_code)]
fn quicksort(a: &mut [isize], l: usize, r: usize) {
    println!("{:?}", a);
    if l < r {
        let q = partition(a, l, r);
        quicksort(a, l, q);
        quicksort(a, q + 1, r);
    } else {
        return;
    }
}

fn partition(a: &mut [isize], l: usize, r: usize) -> usize {
    let v = a[(l + r) / 2];
    let mut i = l;
    let mut j = r;
    
    while i <= j {
        while a[i] < v {
            i += 1;
        }
        while a[j] > v {
            j -= 1;
        }
        if i >= j {
            break;
        }
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
    }

    j
}

#[cfg(test)]
mod tests {

    use super::{
        merge_sort_iterative,
        quicksort
    };

    #[test]
    fn merge_sort_iterative_test() {
        let mut array: [isize; 9] = [77, 89, 74, 68, 70, 49, 5, 62, 51];
        merge_sort_iterative(&mut array);
        assert_eq!(array, [5, 49, 51, 62, 68, 70, 74, 77, 89]);
    }

    #[test]
    fn quicksort_test() {
        let mut array: [isize; 9] = [77, 89, 74, 68, 70, 49, 5, 62, 51];
        let length = array.len();
        quicksort(&mut array, 0, length - 1);
        assert_eq!(array, [5, 49, 51, 62, 68, 70, 74, 77, 89]);
    }
}
