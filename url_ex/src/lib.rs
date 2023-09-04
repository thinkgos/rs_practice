#[cfg(test)]
mod tests {
    use url::{Host, ParseError, Position, Url};

    #[test]
    fn invalid_ip_v6_address() {
        assert!(Url::parse("http://[:::1]") == Err(ParseError::InvalidIpv6Address))
    }
    #[test]
    fn parse_valid_url() -> Result<(), Box<dyn std::error::Error>> {
        let issue_list_url =
            Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;

        assert!(issue_list_url.scheme() == "https");
        assert!(issue_list_url.username() == "");
        assert!(issue_list_url.password() == None);
        assert!(issue_list_url.host_str() == Some("github.com"));
        assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
        assert!(issue_list_url.port() == None);
        assert!(issue_list_url.path() == "/rust-lang/rust/issues");
        assert_eq!(
            issue_list_url
                .path_segments()
                .map(|c| c.collect::<Vec<_>>()),
            Some(vec!["rust-lang", "rust", "issues"])
        );
        assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
        assert!(
            &issue_list_url[Position::BeforePath..]
                == "/rust-lang/rust/issues?labels=E-easy&state=open"
        );
        assert!(issue_list_url.fragment() == None);
        assert!(!issue_list_url.cannot_be_a_base());
        Ok(())
    }
    #[test]
    fn cannot_be_a_base() -> Result<(), Box<dyn std::error::Error>> {
        let data_url = Url::parse("data:text/plain,Hello?World#")?;

        assert!(data_url.cannot_be_a_base());
        assert!(data_url.scheme() == "data");
        assert!(data_url.path() == "text/plain,Hello");
        assert!(data_url.path_segments().is_none());
        assert!(data_url.query() == Some("World"));
        assert!(data_url.fragment() == Some(""));

        Ok(())
    }
}
