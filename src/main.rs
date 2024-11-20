use serde::{Serializer, Deserializer};
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::{DateTime, Utc};
use serde_yaml::to_string as to_yaml;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;
    let result = s.replace("Date: ", "");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();
        assert_eq!(request.stream.is_private, false);
        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.private_tariff.description, "test private tariff");
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].price, 2);
    }
}

fn main() {

    // let user = User {
    //     name: "Augusto".to_string(),
    //     email: "augusto19@example.com".to_string(),
    //     birthdate: "1999-04-09".to_string(),
    // };
    //
    // let json = serde_json::to_string(&user).expect("Serialization error");
    // println!("Serialized JSON: {}", json);
    //
    // let deserialized_user: User = serde_json::from_str(&json).expect("Deserialization error");
    // println!("Deserialized user: {:?}", deserialized_user);


    // let mut file = File::open("request.json").unwrap();
    // let mut json_str = String::new();
    // file.read_to_string(&mut json_str).unwrap();
    //
    // let request: Request = serde_json::from_str(&json_str).unwrap();
    //
    // let yaml_str = to_yaml(&request).unwrap();
    // println!("yaml:\n{}", yaml_str);
    //
    // let toml_str = to_toml(&request).unwrap();
    // println!("toml:\n{}", toml_str);


    let event = Event{
        name: "Schmalgauzen concert".to_string(),
        date: "2024-11-29".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Serialization error");
    println!("Serialized JSON with custom date: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Deserialization error");
    println!("Deserialized event: {:?}", deserialized_event);
}
