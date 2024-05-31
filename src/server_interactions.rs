use std::net::Ipv4Addr;
use wiz_bulb::bulb::Bulb;
use crate::BASE_URL;


pub async fn get_bulb(name: String) -> Result<Bulb, reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/", name);
    reqwest::get(url).await?.json().await
}

pub async fn bulb_on(id: i32) -> Result<(), reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/on/", id);
    reqwest::get(url).await?.text().await.unwrap();
    Ok(())
}

pub async fn bulb_off(id: i32) -> Result<(), reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/off/", id);
    reqwest::get(url).await?.text().await.unwrap();
    Ok(())
}

pub async fn discover_bulbs() -> Result<Vec<Ipv4Addr>, reqwest::Error> {
    let url = format!("{}{}", BASE_URL, "bulb/discover");
    reqwest::get(url).await?.json().await
}
