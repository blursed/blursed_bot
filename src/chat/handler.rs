use slack;

pub struct Handler;

impl slack::EventHandler for Handler {

    fn on_event(&mut self, cli: &slack::RtmClient, event: slack::Event) {
        println!("on_event");

        if let slack::Event::Message(_message) = event {
            // find the testing_blursed_bot channel id
            // TODO Tidy this up
            let testing_channel_id = cli.start_response()
                .channels
                .as_ref()
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

            let _ = cli.sender().send_message(&testing_channel_id, "Hello, I'm a blursed bot");
        }
    }

    fn on_close(&mut self, _cli: &slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, _cli: &slack::RtmClient) {
        println!("on_connect");
    }
}
