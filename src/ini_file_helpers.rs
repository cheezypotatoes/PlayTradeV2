use config::{Config, File};



pub fn access_ini_data(category: &str, data: &str) -> String {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap();

    let key = format!("{}.{}", category, data);
   
    let value: String = settings.get(&key).unwrap();

    value
}
