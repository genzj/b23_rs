use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

pub async fn setup_mock_server() -> MockServer {
    let mock_server = MockServer::start().await;

    // Mock /rlUSCcz to redirect to the video page on the same mock server
    let redirect_url = format!(
        "{}/video/BV1BECcB3EG6?p=1&spm_id_from=333.999.0.0&vd_source=12345",
        mock_server.uri()
    );
    Mock::given(method("GET"))
        .and(path("/rlUSCcz"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", redirect_url.as_str()))
        .mount(&mock_server)
        .await;

    // Mock the video page itself to return a 200 OK so reqwest finishes successfully
    Mock::given(method("GET"))
        .and(path("/video/BV1BECcB3EG6"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Mocked video page"))
        .mount(&mock_server)
        .await;

    mock_server
}
