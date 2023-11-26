use tts::Tts;

fn main() {
    let mut tts = Tts::default().expect("failed to initialize TTS");

    let message = if cfg!(feature = "foobar") {
        "Hello, FooBar World!"
    } else {
        "Hello, Minimal World!"
    };

    tts.speak(message, true).expect("failed to speak message");
}
