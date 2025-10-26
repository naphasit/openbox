#[macro_export]
macro_rules! bundle {
    ( $( $component:expr ),* ) => {
        {
            let mut bundle: Vec<Box<dyn std::any::Any>> = Vec::new();
            $(
                bundle.push(Box::new($component));
            )*
            bundle
        }
    };
}
