@r
identifier hf_dataset_converter;
identifier tokio;
@@
- use hf_dataset_validator::hf_dataset_converter;
+ // use hf_dataset_validator::hf_dataset_converter;
- use tokio;
+ // use tokio;

@r
identifier quiz_logic;
@@
- #[tokio::main]
- async fn main() -> Result<(), Box<dyn std::error::Error>> {
-     quiz_logic::run_quiz().await
- }
+ fn main() -> Result<(), Box<dyn std::error::Error>> {
+     quiz_logic::run_quiz()
+ }