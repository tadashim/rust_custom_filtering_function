fn main() {
    fn download_data(url: &str, callback: impl FnOnce(&str)) {
        println!("Downloading from {}...", url);

        std::thread::sleep(std::time::Duration::from_secs(1));

        let data = format!("Some data from {}", url);

        callback(&data);
    }

    let print_data = |data: &str| {
        println!("Received data: {}", data);
    };
    
    download_data("https://www.rust-lang.org", print_data);
}
