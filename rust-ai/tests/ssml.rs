use rust_ai::azure::{
    types::ssml::{
        tag::{Speak, Voice},
        SSML,
    },
    VoiceName,
};

#[test]
fn test_ssml_generation() {
    let ssml = SSML::default()
        .speak(Speak::new(
            Voice::new(VoiceName::en_US_JennyMultilingualNeural)
                .content("Please don't hurt me! Dad, HELP!!!!".into()),
        ))
        .to_string();

    assert_eq!(
        ssml,
        r#"<speak version="1.0" xml:lang="en-US" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="https://www.w3.org/2001/mstts"><voice name="en-US-JennyMultilingualNeural">Please don't hurt me! Dad, HELP!!!!</voice></speak>"#,
    );
}
