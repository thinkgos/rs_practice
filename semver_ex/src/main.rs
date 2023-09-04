use semver::{BuildMetadata, Prerelease, Version, VersionReq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------  解析 和 设置版本  --------------------");
    let mut parsed_version = Version::parse("0.2.6")?;
    assert_eq!(
        parsed_version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
    );
    parsed_version.patch += 1;
    parsed_version.minor += 1;
    parsed_version.major += 1;
    assert_eq!(parsed_version.to_string(), "1.3.7");
    println!("New release: v{}", parsed_version);

    println!("----------------  解析复杂的版本  --------------------");
    let complex_version = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(complex_version)?;
    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: Prerelease::new("125")?,
            build: BuildMetadata::new("g72ee7853")?,
        }
    );
    assert_eq!(complex_version, parsed_version.to_string());

    println!("----------------  检查版本是否是预发布  --------------------");
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert!(!version_1.pre.is_empty());
    assert!(version_2.pre.is_empty());

    println!("----------------  找到符合要求的版本  --------------------");
    assert_eq!(
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
        Some(Version::parse("1.0.0")?)
    );

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}

fn find_max_matching_version<'a, I>(
    version_req_str: &str,
    iterable: I,
) -> Result<Option<Version>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = &'a str>,
{
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(iterable
        .into_iter()
        .filter_map(|s| Version::parse(s).ok())
        .filter(|s| vreq.matches(s))
        .max())
}
