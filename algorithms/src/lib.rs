mod queue;
mod stack;

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    #[test]
    fn example() {
        let now = SystemTime::now();

        println!("{:?}", now.elapsed().unwrap_or_default())
    }
}
