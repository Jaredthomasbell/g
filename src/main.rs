use tokio::*;
use url::Url;
use thirtyfour::prelude::*;
use tokio::time::sleep;
use std::{error::Error, result};
use tokio::time::timeout;
use std::time::Duration;
use thirtyfour::common::capabilities::chrome::ChromeCapabilities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chrome_capabilities = ChromeCapabilities::new();
    chrome_capabilities.set_headless();
    let driver = WebDriver::new("http://localhost:12345", chrome_capabilities).await?;
    
   //get init URLS 
   let urls: Vec<&str> = vec!["http://stpaulsburkittsville.org/", "http://pvbc1.com/"];
   let sub = subpages(driver, urls, 3).await?;
   Ok(println!("{:?}", sub))

}



async fn subpages(driver: WebDriver, urls: Vec<&str>, depth: usize) ->  Result<Vec<Vec<(String, String)>>, Box<dyn std::error::Error>> {
    let mut result: Vec<Vec<(String, String)>> = Vec::new();
    for url in urls {
        let mut links: Vec<(String, String)> = Vec::new();
        let load_result = timeout(Duration::from_secs(20), driver.goto(&url)).await;
        match load_result{
            Ok(_) => {
                sleep(Duration::from_secs(4)).await;
                let elem = driver.find_element(By::Tag("a")).await?;
                let base_url = Url::parse(url)?;
                if let Some(href) = elem.attr("href").await? { links.push((url.to_string(), href)); }
                    
                } 
        Err(_) => continue,
        }
        result.push(links);
    } 
    Ok((result))
}
