use maxminddb::geoip2;
use std::net::IpAddr;
use std::path::Path;

pub struct GeoLookup {
    reader: Option<maxminddb::Reader<Vec<u8>>>,
}

impl GeoLookup {
    pub fn new(db_path: &str) -> Self {
        let reader = if Path::new(db_path).exists() {
            maxminddb::Reader::open_readfile(db_path).ok()
        } else {
            None
        };

        Self { reader }
    }

    pub fn lookup(&self, ip: &str) -> Option<GeoInfo> {
        let reader = self.reader.as_ref()?;
        let ip_addr: IpAddr = ip.parse().ok()?;

        if let Ok(city) = reader.lookup::<geoip2::City>(ip_addr) {
            let country = city
                .country
                .as_ref()
                .and_then(|c| c.names.as_ref())
                .and_then(|n| n.get("en").map(|s| s.to_string()))
                .unwrap_or_default();
            let city_name = city
                .city
                .as_ref()
                .and_then(|c| c.names.as_ref())
                .and_then(|n| n.get("en").map(|s| s.to_string()))
                .unwrap_or_default();
            let continent = city
                .continent
                .as_ref()
                .and_then(|c| c.names.as_ref())
                .and_then(|n| n.get("en").map(|s| s.to_string()))
                .unwrap_or_default();
            let latitude = city
                .location
                .as_ref()
                .and_then(|l| l.latitude)
                .unwrap_or(0.0);
            let longitude = city
                .location
                .as_ref()
                .and_then(|l| l.longitude)
                .unwrap_or(0.0);

            return Some(GeoInfo {
                country,
                city: city_name,
                continent,
                latitude,
                longitude,
            });
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct GeoInfo {
    pub country: String,
    pub city: String,
    pub continent: String,
    pub latitude: f64,
    pub longitude: f64,
}
