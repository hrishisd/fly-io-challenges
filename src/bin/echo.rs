// cargo b && ../maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10
use anyhow::anyhow;

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
    node.run()
}
