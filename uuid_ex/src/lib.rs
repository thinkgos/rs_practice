#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    #[test]
    fn uuid_test() {
        let id = Uuid::new_v4();
        println!("{}", id);
        println!("{:x}", id.simple());
        println!("{:x}", id.urn());
        println!("{:x}", id.as_braced());
        println!("{:x}", id.as_hyphenated());

        const ID: Uuid = uuid!("78ac65e0-694a-4a48-94a5-5614dfcad9c5");
        println!("{}", ID);
    }
}
