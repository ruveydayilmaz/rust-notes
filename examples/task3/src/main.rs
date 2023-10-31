fn main() {
    let elements = vec![5, 10, 15, 20, 25];
    let condition = FilterCondition { filter_type: 15 };

    let filtered = custom_filter(&elements, &condition);
    println!("result: {:?}", filtered);
}

struct FilterCondition<T> {
    filter_type: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.filter_type
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut filtered = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered.push(item.clone());
        }
    }

    filtered
}