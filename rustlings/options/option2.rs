#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        let optional_word = Some(String::from("rustlings"));

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_word {
            println!("The word is: {}", word);
        } else {
            println!("The optional word doesn't contain anything");
        }

        #[test]
        fn layered_option() {
            let range = 10;
            let mut optional_integers: Vec<Option<i8>> = vec![None];

            for i in 1..10 {
                optional_integers.push(Some(i));
            }

            let mut cursor = range;

            // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
            // You can stack `Option<T>`s into while let and if let

            while let Some(Some(integer)) = optional_integers_vec.pop() {
                println!("current value: {}", integer);
            }
        }
    }
}
