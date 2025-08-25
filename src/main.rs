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
        let peer = Box::new(HttpPeer::new("0.0.0.0:9000", false, "".to_string()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let req = session.req_header();

        if req.method == "OPTIONS" {
            let mut resp = ResponseHeader::build(204, None).unwrap();
            resp.insert_header("Access-Control-Allow-Origin", "*")
                .unwrap();
            resp.insert_header("Access-Control-Allow-Headers", "*")
                .unwrap();
            resp.insert_header("Access-Control-Allow-Methods", "*")
                .unwrap();
            resp.insert_header("Access-Control-Allow-Credentials", "true")
                .unwrap();

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
        upstream_response
            .insert_header("Access-Control-Allow-Origin", "*")
            .unwrap();
        upstream_response
            .insert_header("Access-Control-Allow-Credentials", "true")
            .unwrap();
        Ok(())
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let mut proxy_service = http_proxy_service(&my_server.configuration, Gateway {});
    proxy_service.add_tcp("0.0.0.0:9066");

    my_server.add_service(proxy_service);
    my_server.run_forever();
}
