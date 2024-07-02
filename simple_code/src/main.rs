fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut data = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", data);
    bubble_sort(&mut data);
    println!("Sorted array: {:?}", data);
}
