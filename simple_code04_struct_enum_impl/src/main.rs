#[derive(Debug)]
enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug)]
struct BubbleSort {
    data: Vec<i32>,
    order: SortOrder,
}

impl BubbleSort {
    fn new(data: Vec<i32>, order: SortOrder) -> Self {
        BubbleSort { data, order }
    }

    fn sort(&mut self) {
        let n = self.data.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                match self.order {
                    SortOrder::Ascending => {
                        if self.data[j] > self.data[j + 1] {
                            self.data.swap(j, j + 1);
                        }
                    }
                    SortOrder::Descending => {
                        if self.data[j] < self.data[j + 1] {
                            self.data.swap(j, j + 1);
                        }
                    }
                }
            }
        }
    }

    fn get_sorted_data(&self) -> &Vec<i32> {
        &self.data
    }
}

fn main() {
    let mut bubble_sorter_asc =
        BubbleSort::new(vec![64, 34, 25, 12, 22, 11, 90], SortOrder::Ascending);
    println!(
        "Unsorted array (ascending): {:?}",
        bubble_sorter_asc.get_sorted_data()
    );
    bubble_sorter_asc.sort();
    println!(
        "Sorted array (ascending): {:?}",
        bubble_sorter_asc.get_sorted_data()
    );

    let mut bubble_sorter_desc =
        BubbleSort::new(vec![64, 34, 25, 12, 22, 11, 90], SortOrder::Descending);
    println!(
        "Unsorted array (descending): {:?}",
        bubble_sorter_desc.get_sorted_data()
    );
    bubble_sorter_desc.sort();
    println!(
        "Sorted array (descending): {:?}",
        bubble_sorter_desc.get_sorted_data()
    );
}
