use crate::geolocation::request::Request;
impl<'a> Request<'a> {
    pub fn build(&mut self) -> &'a mut Request {
        let query = format!("key={}", self.key);

        self.query = Some(query);

        let payload = serde_json::json!({
            "wifiAccessPoints": self.wifi_access_points
        });

        self.payload = payload;

        self
    }
}