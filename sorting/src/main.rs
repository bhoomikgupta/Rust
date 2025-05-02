fn selection_sort(arr: &mut Vec<i8>) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

fn bubble_sort(arr: &mut Vec<i8>) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = merge_sort(&mut arr[..mid]);
    let right = merge_sort(&mut arr[mid..]);
    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

fn main() {
    let mut arr: Vec<i8> = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr);
    selection_sort(&mut arr);
    println!("After sorting: {:?}", arr);
    let mut arr2: Vec<i8> = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr2);
    bubble_sort(&mut arr2);
    println!("After sorting: {:?}", arr2);
    let mut arr3: Vec<i32> = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr3);
    let sorted_arr = merge_sort(&mut arr3);
    println!("After sorting: {:?}", sorted_arr);
}
