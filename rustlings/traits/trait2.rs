trait AppendBar {
    fn append_bar(self) -> Self;
}


    // TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
        fn append_bar(mut self) -> Self {
            self.push("Bar".into());
            self
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
