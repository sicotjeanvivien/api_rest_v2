#[macro_export]
macro_rules! routes {
    ( $jwt_service:expr, $( $method:ident $path:literal => $handler:expr ),* $(,)? ) => {{
        let mut router = Router::new(vec![], $jwt_service);
        $(
            let route = Route::new(
                HttpMethod::$method,
                $path.to_string(),
                Box::new($handler),
            );
            router = router.add_route(route);
        )*
        router
    }};
}