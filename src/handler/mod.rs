use chrono::prelude::*;
use colored::*;
use std::time::{Duration, UNIX_EPOCH};

use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

/// 定义接口返回的数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct Weather {
    weather: Vec<WeatherItem>,
    main: Temperature,
    sys: Sys,
    coord: Coord,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherItem {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    feels_like: f64,
    humidity: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    sunrise: i32,
    sunset: i32,
}

/// 定义天气接口的实现
impl Weather {
    pub async fn get(city: &String) -> Result<Weather, ExitFailure> {
        println!("{}", "-----正在获取天气信息, 请稍后...-----".bright_green());
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&lang=zh_cn&units=metric&appid=f11a82bc0ba203a42a65f599bede8025", city);
        let url = Url::parse(url.as_str())?;
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let weather: Weather = serde_json::from_str(&body)?;
        // println!("API Response: {}", body);
        Ok(weather)
    }
}

/// 格式化时间，将时间戳转成指定格式
pub fn format_timestamp(timestamp: i32, format: &str) -> String {
    let time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let datetime = DateTime::<Local>::from(time);
    datetime.format(format).to_string()
}

/// 打印返回结果到控制台
pub fn print_response(resp: &Weather) {
    if let Some(weather) = resp.weather.first() {
        println!(
            "天气：{}\n当前温度：{}℃\n当前最低温：{}℃\n当前最高温：{}℃\n体感温度：{}℃\n湿度：{}%\n今日日出时间：{}\n今日日落时间：{}\n所在经度：{}\n所在纬度：{} \n天气图标：{}",
            weather.description,
            resp.main.temp.to_string().bright_red(),
            resp.main.temp_min,
            resp.main.temp_max,
            resp.main.feels_like,
            resp.main.humidity,
            format_timestamp(resp.sys.sunrise, "%H:%M:%S"),
            format_timestamp(resp.sys.sunset, "%H:%M:%S"),
            resp.coord.lon,
            resp.coord.lat,
            format!("https://openweathermap.org/themes/openweathermap/assets/vendor/owm/img/widgets/{}.png", weather.icon.bright_yellow())
        );
    }
}

/// 时间戳的单测
#[test]
fn test_timestamp_to_time() {
    assert_eq!(
        format_timestamp(1643467428, "%H:%M:%S"),
        "22:43:48".to_string()
    );

    assert_eq!(
        format_timestamp(1643467428, "%Y-%m-%d %H:%M:%S"),
        "2022-01-29 22:43:48".to_string()
    )
}
