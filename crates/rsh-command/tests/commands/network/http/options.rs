use mockito::Server;
use rsh_test_support::{rsh, pipeline};

#[test]
fn http_options_is_success() {
    let mut server = Server::new();

    let _mock = server
        .mock("OPTIONS", "/")
        .with_header("Allow", "OPTIONS, GET")
        .create();

    let actual = rsh!(pipeline(
        format!(
            r#"
        http options {url}
        "#,
            url = server.url()
        )
        .as_str()
    ));

    assert!(!actual.out.is_empty())
}

#[test]
fn http_options_failed_due_to_server_error() {
    let mut server = Server::new();

    let _mock = server.mock("OPTIONS", "/").with_status(400).create();

    let actual = rsh!(pipeline(
        format!(
            r#"
        http options {url}
        "#,
            url = server.url()
        )
        .as_str()
    ));

    assert!(actual.err.contains("Bad request (400)"))
}
