use ndarray::Array2;
use std::collections::HashMap;

pub struct StructuredArray<T> {
    data: Array2<T>,
    column_names: Vec<String>,
    column_indices: HashMap<String, usize>
}

impl<T> StructuredArray<T> {
    pub fn new(data: Array2<T>, column_names: Vec<String>) -> Self {
        let column_indices: HashMap<String, usize> = column_names
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect();

        StructuredArray { data, column_names, column_indices }
    }
}