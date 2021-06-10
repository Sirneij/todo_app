mod to_do;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> =
        to_do_factory("done", "Learn rust programming");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!(
            "This is a pending item with the title: {}",
            item.super_struct.title
        ),
        ItemTypes::Done(item) => println!(
            "This is done item with the title: {}",
            item.super_struct.title
        ),
    }
}
