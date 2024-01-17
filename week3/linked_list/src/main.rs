use linked_list::LinkedList;
use rand::random;
use linked_list::ComputeNorm;

pub mod linked_list;

fn main() {
    let mut list: LinkedList<f64> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for _ in 1..12 {
        list.push_front(random::<f64>());
    }
    let cloned_list = list.clone();
    assert!(list == cloned_list);

    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    println!("cloned list: ");
    println!("{}", cloned_list);

    println!("Computed Norm: {}", list.compute_norm());

    // If you implement iterator trait:
    for val in &list {
       println!("{}", val);
    }
}
