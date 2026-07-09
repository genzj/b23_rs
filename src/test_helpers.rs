use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

pub async fn setup_mock_server() -> MockServer {
    let mock_server = MockServer::start().await;

    // Mock /rlUSCcz to redirect to the video page on the same mock server
    let redirect_url = format!(
        "{}/video/BV1BECcB3EG6?buvid=XUEB1C5EA22C98FB0BF8D33A1BA11D12A2B2D&from_spmid=tm.recommend.0.0&is_story_h5=false&mid=9mRlviQUEP0n%2BrijVaCSQH8FTQ%2FSZMtL1rElX6M3iMo%3D&p=1&plat_id=116&share_from=ugc&share_medium=android_i&share_plat=android&share_session_id=c14d6a0b-152c-453f-9b2e-f776bab6869c&share_source=COPY&share_tag=s_i&spmid=united.player-video-detail.0.0&timestamp=1762786023&unique_k=rlUSCcz&up_id=3493279559584443",
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
