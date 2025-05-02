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

fn main() {
    let mut arr: Vec<i8> = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr);
    selection_sort(&mut arr);
    println!("After sorting: {:?}", arr);
    let mut arr2: Vec<i8> = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr2);
    bubble_sort(&mut arr2);
    println!("After sorting: {:?}", arr2);
}
