use anyhow::anyhow;
use std::io::{self};

use maelstrom_echo::{Body, Message, Node};

struct EchoNode {
    id: String,
}

impl Node for EchoNode {
    fn new(id: String, _all_node_ids: Vec<String>) -> Self {
        EchoNode { id }
    }

    fn handle(&mut self, msg: Message) -> anyhow::Result<()> {
        match msg.body {
            Body::Echo { msg_id, echo } => {
                let response = Message {
                    src: self.id.clone(),
                    dest: msg.src,
                    body: Body::EchoOk {
                        msg_id: 1,
                        in_reply_to: msg_id,
                        echo,
                    },
                };
                self.send(response)
            }
            _ => Err(anyhow!("unexpected message: {msg:?}")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = EchoNode::initialize()?;
    for msg in io::stdin().lines() {
        let line = msg?;
        let msg: Message = serde_json::from_str(&line)?;
        node.handle(msg)?;
    }
    Ok(())
}
