use {
  super::*,
  crate::wallet::{batch, Wallet},
  bitcoincore_rpc::bitcoincore_rpc_json::ListDescriptorsResult,
  shared_args::SharedArgs,
};

pub mod balance;
mod batch_command;
pub mod cardinals;
pub mod create;
pub mod dump;
pub mod inscribe;
pub mod inscriptions;
pub mod mint;
pub mod outputs;
pub mod receive;
pub mod restore;
pub mod resume;
pub mod sats;
pub mod send;
mod shared_args;
pub mod transactions;

#[derive(Debug, Parser)]
pub(crate) struct WalletCommand {
  #[arg(long, default_value = "ord", help = "Use wallet named <WALLET>.")]
  pub(crate) name: String,
  #[arg(long, alias = "nosync", help = "Do not update index.")]
  pub(crate) no_sync: bool,
  #[arg(
    long,
    help = "Use ord running at <SERVER_URL>. [default: http://localhost:80]"
  )]
  pub(crate) server_url: Option<Url>,
  #[arg(
    long,
    help = "Wallet Address"
  )]
  pub(crate) address: Option<String>,
  #[command(subcommand)]
  pub(crate) subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Subcommand {
  #[command(about = "Get wallet balance")]
  Balance,
  #[command(about = "Create inscriptions and runes")]
  Batch(batch_command::Batch),
  #[command(about = "Create new wallet")]
  Create(create::Create),
  #[command(about = "Dump wallet descriptors")]
  Dump,
  #[command(about = "Create inscription")]
  Inscribe(inscribe::Inscribe),
  #[command(about = "List wallet inscriptions")]
  Inscriptions,
  #[command(about = "Mint a rune")]
  Mint(mint::Mint),
  #[command(about = "Generate receive address")]
  Receive(receive::Receive),
  #[command(about = "Restore wallet")]
  Restore(restore::Restore),
  #[command(about = "Resume pending etchings")]
  Resume,
  #[command(about = "List wallet satoshis")]
  Sats(sats::Sats),
  #[command(about = "Send sat or inscription")]
  Send(send::Send),
  #[command(about = "See wallet transactions")]
  Transactions(transactions::Transactions),
  #[command(about = "List all unspent outputs in wallet")]
  Outputs,
  #[command(about = "List unspent cardinal outputs in wallet")]
  Cardinals,
}

impl WalletCommand {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
    eprintln!("Running wallet command: {:?}", self);
    // log::info!("Running wallet command: {:?}", self);
    match self.subcommand {
      Subcommand::Create(create) => return create.run(self.name, &settings),
      Subcommand::Restore(restore) => return restore.run(self.name, &settings),
      _ => {}
    };

    let wallet = match self.address {
        Some(address) => Wallet::build_no_pk(
          address,
          self.no_sync,
          settings.clone(),
          self
              .server_url
              .as_ref()
              .map(Url::as_str)
              .or(settings.server_url())
              .unwrap_or("http://127.0.0.1:80")
              .parse::<Url>()
              .context("invalid server URL")?,
        )?,
      None => Wallet::build(
        self.name.clone(),
        self.no_sync,
        settings.clone(),
        self
            .server_url
            .as_ref()
            .map(Url::as_str)
            .or(settings.server_url())
            .unwrap_or("http://127.0.0.1:80")
            .parse::<Url>()
            .context("invalid server URL")?,
      )?
    };



    match self.subcommand {
      Subcommand::Balance => balance::run(wallet),
      Subcommand::Batch(batch) => batch.run(wallet),
      Subcommand::Dump => dump::run(wallet),
      Subcommand::Inscribe(inscribe) => inscribe.run(wallet),
      Subcommand::Inscriptions => inscriptions::run(wallet),
      Subcommand::Mint(mint) => mint.run(wallet),
      Subcommand::Receive(receive) => receive.run(wallet),
      Subcommand::Resume => resume::run(wallet),
      Subcommand::Sats(sats) => sats.run(wallet),
      Subcommand::Send(send) => send.run(wallet),
      Subcommand::Transactions(transactions) => transactions.run(wallet),
      Subcommand::Outputs => outputs::run(wallet),
      Subcommand::Cardinals => cardinals::run(wallet),
      Subcommand::Create(_) | Subcommand::Restore(_) => unreachable!(),
    }
  }
}
