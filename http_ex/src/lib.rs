#[cfg(test)]
mod tests {
    use http::header::{self, HeaderName};
    use http::HeaderValue;

    #[test]
    fn header_name() {
        let name: HeaderName = header::ACCEPT;
        assert_eq!(name.as_str(), "accept");

        let name: HeaderName = "Accept".parse().unwrap();
        assert_eq!(name, header::ACCEPT);
    }
    #[test]
    fn header_value() {
        let value = HeaderValue::from_static("text/html");
        assert_eq!(value.as_bytes(), b"text/html");

        let value = "text/html";
        let value = value.parse::<HeaderValue>().unwrap();

        assert_eq!(value.to_str().unwrap(), "text/html")
    }
}
