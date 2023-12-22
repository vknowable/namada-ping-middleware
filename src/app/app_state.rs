use namada_sdk::{
  Namada, NamadaImpl, wallet::fs::FsWalletUtils, masp::fs::FsShieldedUtils, io::NullIo,
};
use tendermint_rpc::{HttpClient, Url};

pub struct AppState {
  // http_client: HttpClient,
  namada_impl: NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
}

impl AppState {
  pub async fn new(rpc_url: Url) -> Self {
      // setup namada_impl
      let http_client: HttpClient = HttpClient::new(rpc_url).unwrap();
      let wallet = FsWalletUtils::new("wallet".into());
      let shielded_ctx = FsShieldedUtils::new("masp".into());
      let null_io = NullIo;
      Self {
          namada_impl: NamadaImpl::new(http_client, wallet, shielded_ctx, null_io).await.unwrap(),
      }
  }

  pub fn get_client(&self) -> &HttpClient {
      &self.namada_impl.client()
  }
}
