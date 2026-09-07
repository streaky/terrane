#[derive(Clone)]
struct TerraneThrowableLogValue {
    value: TerraneError,
}

impl LogValueProtocol for TerraneThrowableLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }

    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }

    fn render(&self) -> DocumentValue {
        make_document_string(self.value.render())
    }
}

fn terrane_log_error(value: TerraneError) -> LogValue {
    LogValue(Box::new(TerraneThrowableLogValue { value }))
}
