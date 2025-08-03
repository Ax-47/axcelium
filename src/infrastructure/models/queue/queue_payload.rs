pub trait QueueOperation {
    fn operation(&self) -> &str;
}
