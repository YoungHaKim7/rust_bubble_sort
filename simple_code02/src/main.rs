#[derive(Debug)]
struct Young {
    name: String,
}

enum Test {
    Right,
    Left,
}

impl Young {
    fn new(&self) -> Self {
        Self {
            name: "".to_string(),
        }
    }

    fn push(&mut self, _: Test) -> Test {
        Test::Right
    }
}

fn print_array01<T: std::fmt::Display>(array: &[T]) {
    print!("[");
    for (i, elem) in array.iter().enumerate() {
        print!("{}", elem);

        if i != array.len() - 1 {
            print!(",");
        }
    }
    println!("]");
}

fn main() {
    let mut sortable: [i32; 5] = [5, 8, 2, 7, 6];
    let length = sortable.len();
    let mut swapped = true;

    println!("bubble sort ~~~~start");
    print_array01(&sortable);

    while swapped {
        swapped = false;
        for i in 1..length {
            let previous_element = sortable[i - 1];
            let current_element = sortable[i];

            if previous_element > current_element {
                sortable.swap(i - 1, i);
                swapped = true;
            }
        }
    }

    println!("bubble sort ~~~~end");
    print_array01(&sortable);
    let my_name = Young {
        name: "GYoung".to_string(),
    };

    println!("{:?}", &my_name);

    println!("new {:?}", &my_name.new());
}
