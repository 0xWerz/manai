use reqwest::Client;

pub struct OpenAiClient {
    client: Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();
        Ok(OpenAiClient {
            client,
            api_key: api_key.to_owned(),
        })
    }

    pub async fn generate_response(
        &self,
        user_input: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.openai.com/v1/chat/completions";
        let model = "gpt-3.5-turbo";

        let payload = serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": user_input}]
        });

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;

        if let Some(error) = response_json.get("error") {
            let error_message = error["message"].as_str().unwrap_or("Unknown error");
            return Err(error_message.into());
        }

        let choices = response_json["choices"]
            .as_array()
            .ok_or("Invalid API response: 'choices' field missing or not an array")?;
        let ai_response = choices
            .get(0)
            .and_then(|choice| choice["message"]["content"].as_str())
            .unwrap_or("No response")
            .to_owned();

        Ok(ai_response)
    }
}
