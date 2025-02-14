use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, RpcResult},
	ws_server::{SubscriptionSink, WsServerBuilder},
};
use std::{net::SocketAddr, sync::mpsc::channel};

#[rpc(server)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "sub", item = String)]
	fn sub(&self) -> RpcResult<()>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
		Ok(42u16)
	}

	fn sync_method(&self) -> RpcResult<u16> {
		Ok(10u16)
	}

	fn sub(&self, mut sink: SubscriptionSink) -> RpcResult<()> {
		sink.send(&"Response_A")?;
		sink.send(&"Response_B")
	}
}

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();
		let server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start(RpcServerImpl.into_rpc()).await
		});
	});

	server_started_rx.recv().unwrap()
}

#[tokio::main]
async fn main() {
	let _server_addr = websocket_server().await;
}
