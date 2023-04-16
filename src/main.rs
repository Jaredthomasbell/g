
use url::Url;
use thirtyfour::prelude::*;
use tokio::time::sleep;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::{Attr};
use std::collections::HashSet;
use std::iter::FromIterator;
use tokio::time::timeout;
use select::predicate::{Name};
use std::time::Duration;
use thirtyfour::common::capabilities::chrome::ChromeCapabilities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
   //get init URLS 
   let urls: Vec<&str> = vec!["http://stpaulsburkittsville.org/", "http://pvbc1.com/"];
   let mut result = Vec::new();

   for url in  urls {
        match find_subpages(url).await{
            Ok(subpages) => {
                result.push(subpages);

            }
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(println!("{:?}", result))
}



async fn subpages(driver: WebDriver, urls: Vec<&str>, _depth: usize) ->  Result<Vec<Vec<(String, String)>>, Box<dyn std::error::Error>> {
    let mut result: Vec<Vec<(String, String)>> = Vec::new();
    for url in urls {
        let mut links: Vec<(String, String)> = Vec::new();
        let load_result = timeout(Duration::from_secs(20), driver.goto(&url)).await;
        match load_result{
            Ok(_) => {
                sleep(Duration::from_secs(4)).await;
                let elem = driver.find_element(By::Tag("a")).await?;
                let _base_url = Url::parse(url)?;
                if let Some(href) = elem.attr("href").await? { links.push((url.to_string(), href)); }
                    
                } 
        Err(_) => continue,
        }
        result.push(links);
    } 
    Ok(result)
}

async fn find_subpages(url: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let root_url = Url::parse(url)?;
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::from_iter(vec![url.to_string()]);

    for _ in 0..2 {
        let mut new_pages = HashSet::new();
        for url in to_visit {
            if visited.contains(&url) {
                continue;
            }
            visited.insert(url.clone());

            let response = Client::new().get(&url).send()?;
            tokio::time::sleep(Duration::from_secs(2)).await;
            let document = Document::from_read(response.text()?.as_bytes())?;

            for link in document.find(Attr("href", ())) {
                if let Some(link_url) = link.attr("href") {
                    let base_url = Url::parse(&url)?;
                    if let Ok(abs_url) = base_url.join(link_url) {
                        // Check if the origin of the absolute URL is the same as the root URL
                        if abs_url.origin() == root_url.origin() {
                            let abs_url_string = abs_url.into_string();
                            // Check if the URL does not end with ".jpg"
                            if !abs_url_string.ends_with(".jpg") {
                                new_pages.insert(abs_url_string);
                            }
                        }
                    }
                }
            }
        }
        to_visit = new_pages;
    }

    Ok(visited)
}