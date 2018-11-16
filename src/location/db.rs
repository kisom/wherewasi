pub fn get_database() -> String {
    match std::env::var("WHEREWASI_DATABASE") {
        Ok(val) => val,
        Err(_)  => "wherewasi.db".to_string()
    }
}
