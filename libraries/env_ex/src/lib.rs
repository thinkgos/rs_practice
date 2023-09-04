#[cfg(test)]
mod tests {
    use std::env;
    use std::env::VarError;
    #[test]
    fn functions_vars() {
        let home = env::var("HOME");
        assert!(home.is_ok());

        let invalid_var_key = env::var("xxx");
        assert_eq!(invalid_var_key, Err(VarError::NotPresent));
    }

    #[test]
    fn function_set_vars() {
        env::set_var("app_name", "env_ex");

        let app_name = env::var("app_name");
        assert_eq!(app_name.unwrap_or_default(), "env_ex");

        env::remove_var("app_name")
    }

    #[test]
    fn function() {
        println!("{:?}", env::current_dir().unwrap());
        println!("{:?}", env::current_exe().unwrap());
    }
}
