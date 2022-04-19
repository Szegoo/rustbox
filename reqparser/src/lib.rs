pub fn get_url_path(req: &str) -> String {
    let slash_indx = req.find("/").unwrap();
    let new_req: String = req.chars().skip(slash_indx+1).collect();
    let slash2_indx = new_req.find("/").unwrap();
    let path: String = new_req.chars().take(slash2_indx - slash_indx).collect();
    path
}

pub fn get_file_extension(path: &String) -> String {
    let dot_indx = path.find(".").unwrap();
    path.chars().skip(dot_indx+1).collect()
}

pub fn get_headers(extension: String) -> [String; 3] {
    let ctype = get_req_ctype(&extension); 
    let content = format!("Content-Type: {}/{}", ctype, extension);
    let headers = [
        String::from("HTTP/1.1 200 OK"),
        content,
        String::from("\r\n")
    ];
    headers
}

pub fn get_req_ctype(extension: &String) -> String {
    match extension.as_str() {
        "pdf" => String::from("application"),
        "jpeg" | "jpg" | "png" => String::from("image"),
        _ => panic!("Unknown content type"), 
    }
}