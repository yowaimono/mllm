pub mod llmchain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub use llmchain::llm::{
    DeepSeekConfig,
    DeepSeekLLM,
    Message,
    MessageRole,
};
