#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: Option<String>,
    }

    // json,yaml 序列化和反序列化
    #[test]
    fn json_serde() {
        let point = Point {
            x: 1,
            y: Some("2".to_owned()),
        };

        let json_serialized = serde_json::to_string(&point).unwrap();
        assert_eq!(json_serialized, r#"{"x":1,"y":"2"}"#);

        let json = r#"{"x":1,"y":"2"}"#;
        let json_deserialized: Point = serde_json::from_str(json).unwrap();
        assert_eq!(json_deserialized, point);
    }
    // json,yaml 序列化和反序列化
    #[test]
    fn yaml_serde() {
        // yaml
        let point = Point {
            x: 11,
            y: Some("12".to_owned()),
        };

        let yaml_serialized = serde_yaml::to_string(&point).unwrap();
        assert_eq!(yaml_serialized, "x: 11\ny: '12'\n");

        let yaml = "x: 11\ny: \"12\"\n";
        let yaml_deserialized: Point = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(point, yaml_deserialized);
    }
}
