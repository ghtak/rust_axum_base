#[cfg(test)]
mod tests {
    use axum::http::response;

    use serde_json::json;

    #[tokio::test]
    async fn it_works() {

        let client = reqwest::Client::new();
        let before_resp =  client.get("http://127.0.0.1:3001/state").send().await.unwrap().text().await.unwrap();
        
        let jsvalue = json!({ "datax": format!("{}-", before_resp )});
        let res = client.post("http://127.0.0.1:3001/state")
            .json(&jsvalue)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();           
        println!("{:?}", res);

        let after_resp =  client.get("http://127.0.0.1:3001/state").send().await.unwrap().text().await.unwrap();
        println!("{} {}", before_resp, after_resp);
    }
}
