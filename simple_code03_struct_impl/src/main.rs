#[derive(Debug)]
struct BubbleSort {
    data: Vec<i32>,
}

impl BubbleSort {
    fn new(data: Vec<i32>) -> Self {
        BubbleSort { data }
    }

    fn sort(&mut self) {
        let n = self.data.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if self.data[j] > self.data[j + 1] {
                    self.data.swap(j, j + 1);
                }
            }
        }
    }

    fn get_sorted_data(&self) -> &Vec<i32> {
        &self.data
    }
}

fn main() {
    let mut bubble_sorter = BubbleSort::new(vec![64, 34, 25, 12, 22, 11, 90]);
    println!("Unsorted array: {:?}", bubble_sorter.get_sorted_data());
    bubble_sorter.sort();
    println!("Sorted array: {:?}", bubble_sorter.get_sorted_data());
}
