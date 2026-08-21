use oxide_generation::TypedGenerationKind;
use oxide_generation_macros::impl_typed_generation_kinds;

impl_typed_generation_kinds! {
    kinds = {
        HTTPClient = {},
        XMLParser = {},
        APIKey = {},
        IOHandler = {},
        UserAccount = {},
        ProjectTask = {},
    }
}

fn main() {
    // Test that complex names are handled correctly
    let _http_client_tag = HTTPClientKind::TAG;
    let _xml_parser_tag = XMLParserKind::TAG;
    let _api_key_tag = APIKeyKind::TAG;
    let _io_handler_tag = IOHandlerKind::TAG;
    let _user_account_tag = UserAccountKind::TAG;
    let _project_task_tag = ProjectTaskKind::TAG;

    // Test type aliases exist
    let _http_client_generation: HTTPClientGeneration;
    let _xml_parser_generation: XMLParserGeneration;
    let _api_key_generation: APIKeyGeneration;
    let _io_handler_generation: IOHandlerGeneration;
    let _user_account_generation: UserAccountGeneration;
    let _project_task_generation: ProjectTaskGeneration;

    // Test that complex names are properly snake_cased
    assert_eq!(HTTPClientKind::TAG.as_str(), "http_client");
    assert_eq!(XMLParserKind::TAG.as_str(), "xml_parser");
    assert_eq!(APIKeyKind::TAG.as_str(), "api_key");
    assert_eq!(IOHandlerKind::TAG.as_str(), "io_handler");
    assert_eq!(UserAccountKind::TAG.as_str(), "user_account");
    assert_eq!(ProjectTaskKind::TAG.as_str(), "project_task");
}
