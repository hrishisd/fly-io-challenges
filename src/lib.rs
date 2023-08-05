use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        in_reply_to: usize,
        id: String,
    },
}

pub trait Node {
    fn new(node_id: String, all_node_ids: Vec<String>) -> Self;

    fn initialize() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        eprintln!("received {buffer}");
        let message: Message = serde_json::from_str(&buffer)?;
        let node = {
            if let Message {
                src: init_src,
                dest: _,
                body:
                    Body::Init {
                        msg_id,
                        node_id,
                        node_ids,
                    },
            } = message
            {
                let node = Self::new(node_id.clone(), node_ids);
                let response = Message {
                    src: node_id,
                    dest: init_src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                };
                node.send(response)?;
                Ok(node)
            } else {
                Err(anyhow!("expected init message: {message:?}"))
            }
        }?;
        Ok(node)
    }

    fn handle(&mut self, msg: Message) -> anyhow::Result<()>;

    fn send(&self, msg: Message) -> anyhow::Result<()> {
        let msg = serde_json::to_string(&msg)?;
        println!("{msg}");
        io::stdout().flush()?;
        Ok(())
    }

    fn run(&mut self) -> anyhow::Result<()> {
        for msg in io::stdin().lines() {
            let line = msg?;
            let msg: Message = serde_json::from_str(&line)?;
            self.handle(msg)?;
        }
        Ok(())
    }
}

#[test]
fn test_deserialize_echo() {
    let message = r#"{"src": "c1","dest": "n1","body": {"type": "echo", "msg_id": 1,"echo": "Please echo 35"}}"#;
    let message: Message = serde_json::from_str(message).unwrap();
    println!("{message:?}");
}

#[test]
fn test_deserialize_init() {
    let message = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"init","node_id":"n0","node_ids":["n0"],"msg_id":1}}"#;
    let message: Message = serde_json::from_str(message).unwrap();
    println!("{message:?}");
}
