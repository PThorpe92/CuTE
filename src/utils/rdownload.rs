
// ***** Recursive Downloading ******
//
// This is intended to replace the functionality of wget Recursive
// donwloading for windows. So we're not Msys2 / WSL only.
//

use std::path::Path;
use scraper::{Html, Selector};
use url::Url;
use std::collections::HashSet;


fn slow_filename_sanitization(url:String) -> String {

    let s = url.replace("/", "_")
        .replace(":", "_")
        .replace("?", "_")
        .replace("=", "_")
        .replace("&", "_")
        .replace(".", "_")
        .replace("https", "")
        .replace("http", "");
    s

}

async fn download_html(url:&Url) -> Result<String, ()> {
    

    let resp_result = reqwest::get(url.as_str()).await;
    
    match resp_result {
        Ok(response) => {
            
            // Response Was Good 
            // Return The Body As string 
            let response_body = response.text().await.unwrap();

            Ok(response_body)

        }
        Err(err) => {
            // We Silently Error, And End Up Printing The Error Out In The Output
            eprintln!("[-] Failed To Connect To {:?}", url);
            Ok(format!("[-] Failed To Connect: {}", err))
        }
    }


}


fn extract_links(html:&str, base_url:&Url) -> HashSet<Url> {
    // Parse DOM
    let document = Html::parse_document(html);
    // Construct Link Selector
    let selector = Selector::parse("a").unwrap();
    // Create Hashset For Urls
    let mut urls = HashSet::new();


    // Iterate Thru All Links In DOM Add The Href To The Base URL And Add It TO The Set Of URLS
    // TODO: handle case where href already has full URL in it
    for element in document.select(&selector) {
        println!("Parsed Link Element = {:?}", element.value().attr("href"));
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = base_url.join(href) {
                println!("Added Walkable URL = {:?}", url);
                urls.insert(url);
            }
        }
    }

    urls
}


pub async fn rdownload(url:&str, depth:usize, output_dir:&Path) {
    let url = Url::parse(url).expect("[-] Failed To Parse URL");
    let output_dir = output_dir.to_path_buf();

    if !output_dir.is_dir() {
        // If directory doesnt exist, create it
        std::fs::create_dir_all(&output_dir).expect("[-] Failed To Create Output Directory");
    }

    // Get Initial Page
    let intial_html = download_html(&url).await.unwrap();
    // Get Initial Set Of Links
    let mut urls = extract_links(&intial_html, &url);

    // Save Initial Page
    // Create File Name From Entire URL
    let filename = slow_filename_sanitization(url.to_string());
    let path = output_dir.join(filename);
    std::fs::write(&path, intial_html).expect("[-] Failed To Write File");

    // If Depth is Greater Than 0, Iterate Thru All Links And Download Them
    if depth > 0 {
        // For Each Layer Of Depth
        // Iterate Thru All Links
        // Download Them
        // Save Them
        for _ in 0..depth {
            let mut new_urls = HashSet::new();
            for url in urls {
                println!("Downloading: {:?}", url);
                let html = download_html(&url).await.unwrap();
                let filename = slow_filename_sanitization(url.to_string());
                let mut path = output_dir.join(filename);
                std::fs::write(&path, html.clone()).expect("[-] Failed To Write File");
                new_urls.extend(extract_links(&html, &url));
            }
            urls = new_urls;
        }
    } else {
        return;
    }


}


#[cfg(test)]

mod tests {

    use super::*;

    #[tokio::test]
    async fn r_download() {
        let url = "https://www.uma.edu/";
        let depth = 2;
        let output_dir = Path::new("./test/");
        rdownload(url, depth, output_dir).await;
    }


}