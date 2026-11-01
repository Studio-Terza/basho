mod call;
mod claim;

use std::fmt;
use std::net::TcpStream;
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};
use tungstenite::WebSocket;
use tungstenite::stream::MaybeTlsStream;

use call::Call;
use claim::Claim;

pub struct DevTools {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    call_index: usize,
    session: Option<String>,
    target: Option<String>,
    created: Option<String>,
}

impl DevTools {
    const BLANK: &str = "about:blank";
    const PAGE: &str = "page";
    const BACKOFF: [Duration; 3] = [
        Duration::ZERO,
        Duration::from_millis(50),
        Duration::from_millis(250),
    ];
    const PATIENCE: Duration = Duration::from_secs(60);
    const SCHEME: &str = "ws://";
    const TARGET: &str = "/targetId";
    const SESSION: &str = "/sessionId";
    const VALUE: &str = "/result/value";

    pub fn connect(endpoint: &str) -> DevTools {
        let address = endpoint
            .trim_start_matches(Self::SCHEME)
            .split('/')
            .next()
            .unwrap_or_default();
        let socket = Self::BACKOFF
            .iter()
            .find_map(|wait| {
                std::thread::sleep(*wait);
                TcpStream::connect(address)
                    .ok()
                    .filter(|stream| stream.set_read_timeout(Some(Self::PATIENCE)).is_ok())
                    .and_then(|stream| {
                        tungstenite::client(endpoint, MaybeTlsStream::Plain(stream)).ok()
                    })
            })
            .map(|(socket, _)| socket)
            .expect("Chrome takes a DevTools connection");
        DevTools {
            socket,
            call_index: 0,
            session: None,
            target: None,
            created: None,
        }
    }

    pub fn attached(mut self) -> DevTools {
        let claim = Claim::held();
        let targets = self
            .replied("Target.getTargets", &json!({}))
            .unwrap_or_default();
        let free = serde_json::from_str::<Value>(&targets)
            .ok()
            .and_then(|reply| {
                reply["targetInfos"].as_array()?.iter().find(|info| {
                    info["type"] == Self::PAGE
                        && info["url"] == Self::BLANK
                        && info["attached"] == json!(false)
                })?["targetId"]
                    .as_str()
                    .map(str::to_owned)
            });
        let created = free
            .is_none()
            .then(|| {
                self.called(
                    "Target.createTarget",
                    &json!({ "url": Self::BLANK }),
                    Self::TARGET,
                )
            })
            .flatten();
        let target = free.or_else(|| created.clone());
        let session = self.called(
            "Target.attachToTarget",
            &json!({ "targetId": target, "flatten": true }),
            Self::SESSION,
        );
        drop(claim);
        DevTools {
            session,
            target,
            created,
            ..self
        }
    }

    pub fn fresh_on(endpoint: &str) -> Option<DevTools> {
        let mut tools = DevTools::connect(endpoint);
        let created = tools.called(
            "Target.createTarget",
            &json!({ "url": Self::BLANK }),
            Self::TARGET,
        )?;
        let session = tools.called(
            "Target.attachToTarget",
            &json!({ "targetId": created, "flatten": true }),
            Self::SESSION,
        )?;
        Some(DevTools {
            session: Some(session),
            target: Some(created.clone()),
            created: Some(created),
            ..tools
        })
    }

    pub fn replied(&mut self, method: &str, params: &impl Serialize) -> Option<String> {
        self.call_index += 1;
        let call = Call::indexed(self.call_index, method, params, self.session.as_deref());
        self.socket.send(call.message()).ok()?;
        let index = json!(self.call_index);
        std::iter::repeat_with(|| self.socket.read())
            .map_while(Result::ok)
            .filter_map(|reply| reply.into_text().ok())
            .filter_map(|text| serde_json::from_str::<Value>(&text).ok())
            .find(|reply| reply["id"] == index)
            .map(|reply| reply["result"].to_string())
    }

    pub fn called(
        &mut self,
        method: &str,
        params: &impl Serialize,
        answer: &str,
    ) -> Option<String> {
        serde_json::from_str::<Value>(&self.replied(method, params)?)
            .ok()?
            .pointer(answer)?
            .as_str()
            .map(str::to_owned)
    }

    pub fn evaluated(&mut self, expression: &str) -> Option<String> {
        self.called(
            "Runtime.evaluate",
            &json!({ "expression": expression, "returnByValue": true }),
            Self::VALUE,
        )
    }

    pub fn awaited(&mut self, expression: &str) -> Option<String> {
        self.called(
            "Runtime.evaluate",
            &json!({ "expression": expression, "returnByValue": true, "awaitPromise": true, "userGesture": true }),
            Self::VALUE,
        )
    }

    pub fn emulated(&mut self, metrics: &impl Serialize) -> Option<()> {
        self.replied("Emulation.setDeviceMetricsOverride", metrics)
            .filter(|result| result.starts_with('{'))
            .map(drop)
    }

    pub fn close(&mut self) {
        self.session = None;
        let created = self.created.take();
        created.map(|target| self.replied("Target.closeTarget", &json!({ "targetId": target })));
    }
}

impl fmt::Display for DevTools {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.target.as_deref().unwrap_or(Self::BLANK))
    }
}
