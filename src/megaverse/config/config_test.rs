#[cfg(test)]
mod tests {
    use crate::megaverse::config::handler::{AppProtocol, Config, Resources};

    #[test]
    fn test_json_parse() {
        let data = r#"
        {
            "host": "www.example.com",
            "protocol": "https",
            "api_endpoint": "/api",
            "port": 80,
            "parallel": true,
            "candidate_id": "id1",
            "resources": {
                "soloons": "/soloons",
                "polyanets": "/polyanets",
                "comeths": "/comeths",
                "goal": ""
            }
        }"#;

        let cfg: Config = serde_json::from_str(data).unwrap();

        let expected = Config {
            host: "www.example.com".to_string(),
            protocol: AppProtocol::Https,
            api_endpoint: "/api".to_string(),
            port: 80u32,
            parallel: true,
            candidate_id: "id1".to_string(),
            resources: Resources {
                polyanets: "/polyanets".to_string(),
                comeths: "/comeths".to_string(),
                goal: "".to_string(),
                soloons: "/soloons".to_string(),
            },
        };

        assert_eq!(cfg, expected);
    }
    #[test]
    fn test_default() {
        let expected = Config {
            host: "".to_string(),
            protocol: AppProtocol::Https,
            api_endpoint: "".to_string(),
            port: 0u32,
            parallel: false,
            candidate_id: "".to_string(),
            resources: Resources {
                polyanets: "".to_string(),
                comeths: "".to_string(),
                goal: "".to_string(),
                soloons: "".to_string(),
            },
        };
        assert_eq!(expected, Default::default())
    }
}
