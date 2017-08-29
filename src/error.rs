error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
        Xml(::serde_xml_rs::Error);
    }

    errors {
        AuthenticationError {
            description("failed to authenticate")
        }
    }
}
