pub fn dispatch<F>(label: String, enabled: bool, callback: F) -> String
where
    F: Fn(String, bool) -> String + Send + Sync + 'static,
{
    callback(label, enabled)
}
