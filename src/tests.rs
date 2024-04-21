#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn get_post_state() {
        let client = reqwest::Client::new();
        let data =  client.get("http://127.0.0.1:3001/state").send().await.unwrap().text().await.unwrap();
        let jsvalue = json!({ "data": "new_data"});
        let res = client.post("http://127.0.0.1:3001/state")
            .json(&jsvalue)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();           
        println!("{:?}", res);

        let after_data =  client.get("http://127.0.0.1:3001/state").send().await.unwrap().text().await.unwrap();
        println!("{} {}", data, after_data);
    }
}
