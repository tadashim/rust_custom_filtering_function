struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.condition == *item
    }
}

fn custom_filter<T>(collection: Vec<T>, filter: &FilterCondition<T>) -> Vec<T> 
where 
    T: PartialEq,
{
    let mut result = Vec::new();
    for item in collection {
        if filter.is_match(&item) {
            result.push(item);
        }
    }
    result
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { condition: 2 };
    let result = custom_filter(collection, &filter_condition);
    println!("Filtered Collection: {:?}", result);
}
