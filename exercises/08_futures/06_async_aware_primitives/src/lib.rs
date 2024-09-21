/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
// use std::sync::mpsc;
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        println!("pong: before receive request from client");
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(1);
            println!("pong: before send response to client");
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await.unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    //use std::sync::mpsc;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        println!("ping: before creating a std channel");
        let (sender, receiver) = mpsc::channel(1);
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        println!("ping: before sender.send");
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await
            .unwrap();
        println!("ping: before spawn a new thread for pong");
        tokio::spawn(pong(receiver));
        println!("ping: before receiving a response from server");
        let answer = response_receiver.recv().await.unwrap().payload;
        println!("ping: after received a response");
        assert_eq!(answer, "pong");
    }
}
