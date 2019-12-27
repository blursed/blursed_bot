use slack;
use slack_api;

pub struct Handler;

impl slack::EventHandler for Handler {

    fn on_event(&mut self, client: &slack::RtmClient, event: slack::Event) {
        if let slack::Event::Message(message) = event {
            match *message {
                slack_api::Message::Standard(message_standard) => {
                    let testing_channel_id = client.start_response().channels.as_ref()
                        .and_then(|channels| {
                                      channels
                                          .iter()
                                          .find(|chan| match chan.name {
                                                    None => false,
                                                    Some(ref name) => name == "testing_blursed_bot",
                                                })
                                  })
                        .and_then(|chan| chan.id.as_ref())
                        .expect("testing_blursed_bot channel not found");
                    let message_text = &message_standard.text.unwrap();
                    let botmsg = format!("Hello I'm a blursed bot, and you typed: {}", &message_text);
                    let _ = client.sender().send_message(&testing_channel_id, &botmsg);
                },
                _ => (),
            }
        }
    }

    fn on_close(&mut self, _client: &slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, _client: &slack::RtmClient) {
        println!("on_connect");
    }
}
