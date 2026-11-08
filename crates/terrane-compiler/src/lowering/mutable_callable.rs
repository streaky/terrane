//! One owned mutable callable implementation for ordinary tuple arguments and
//! invocation-local borrowed input shapes. Only the type binders and argument
//! type vary; capture bounds, locking and copy semantics have one owner.
pub(super) fn runtime(name: &str, types: &str, arguments: &str) -> String {
    format!(
        r#"
        trait {name}Body<{types}>: Send {{
            fn call(&mut self, arguments: {arguments}) -> Output;
            fn clone_box(&self) -> std::boxed::Box<dyn {name}Body<{types}>>;
        }}
        impl<{types}, Function> {name}Body<{types}> for Function
        where Function: FnMut({arguments}) -> Output + Clone + Send + 'static {{
            fn call(&mut self, arguments: {arguments}) -> Output {{ self(arguments) }}
            fn clone_box(&self) -> std::boxed::Box<dyn {name}Body<{types}>> {{ std::boxed::Box::new(self.clone()) }}
        }}
        pub struct {name}<{types}> {{
            body: std::sync::Mutex<std::boxed::Box<dyn {name}Body<{types}>>>,
        }}
        impl<{types}> {name}<{types}> {{
            pub fn new<Function>(function: Function) -> Self
            where Function: FnMut({arguments}) -> Output + Clone + Send + 'static {{
                Self {{ body: std::sync::Mutex::new(std::boxed::Box::new(function)) }}
            }}
            // Semantic mode checking provides exclusive invocation. Clone forks
            // the environment instead of sharing the callable's mutex/state.
            pub fn call(&self, arguments: {arguments}) -> Output {{
                self.body.lock().expect("mutable callable lock poisoned").call(arguments)
            }}
        }}
        impl<{types}> Clone for {name}<{types}> {{
            fn clone(&self) -> Self {{
                let body = self.body.lock().expect("mutable callable lock poisoned").clone_box();
                Self {{ body: std::sync::Mutex::new(body) }}
            }}
        }}
        "#
    )
}
