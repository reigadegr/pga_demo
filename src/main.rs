#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious
)]
use async_trait::async_trait;
use pingora::{http::ResponseHeader, prelude::*};

pub struct Gateway {}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = Box::new(HttpPeer::new("127.0.0.1:9190", false, String::new()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let req = session.req_header();

        if req.method == "OPTIONS" {
            let mut resp = ResponseHeader::build(204, None)?;
            resp.insert_header("Access-Control-Allow-Origin", "*")?;
            resp.insert_header("Access-Control-Allow-Headers", "*")?;
            resp.insert_header("Access-Control-Allow-Methods", "*")?;
            resp.insert_header("Access-Control-Allow-Credentials", "true")?;

            session.write_response_header(Box::new(resp), false).await?;
            return Ok(true);
        }

        Ok(false)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_response.insert_header("Access-Control-Allow-Origin", "*")?;
        upstream_response.insert_header("Access-Control-Allow-Credentials", "true")?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut my_server = Server::new(None)?;
    my_server.bootstrap();

    let mut proxy_service = http_proxy_service(&my_server.configuration, Gateway {});
    proxy_service.add_tcp("0.0.0.0:9066");

    my_server.add_service(proxy_service);
    my_server.run_forever();
}
