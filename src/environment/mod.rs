use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Clap;
use mongodb::{options::ClientOptions, Client, Database};
use warp::Filter;

use argon::Argon;
mod argon;

#[derive(Clone, Debug)]
pub struct Environment {
    db_pool: Client,
    config: Args,
    argon: Argon,
}

#[derive(Clone, Clap, Debug)]
#[clap(
name = "demo-api",
rename_all = "kebab-case",
rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(short, long)]
    debug: bool,

    #[clap(required = true, short = 'D', long, env)]
    db_url: String,

    #[clap(required = true, long, env)]
    db_name: String,

    #[clap(required = true, long, env)]
    jwt_secret: String,
    #[clap(required = true, long, env)]
    argon_secret: String,
    #[clap(long, env)]
    argon_iterations: Option<u32>,
    #[clap(long, env)]
    argon_memory_size: Option<u32>,

    #[clap(default_value = "0.0.0.0:8080", env)]
    pub host: SocketAddr,
}

impl Environment {
    pub async fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let Args {
            db_url,
            db_name,
            ..
        } = &args;

        println!("DB URL: {:?}", &db_url);
        let mut db_config = ClientOptions::parse(db_url).await?;
        db_config.app_name = Some(String::from(db_name));
        db_config.server_selection_timeout = Some(std::time::Duration::new(5, 0));
        let db_pool = Client::with_options(db_config)?;

        let argon = Argon::new(&args);
        Ok(Self {
            db_pool,
            config: args,
            argon,
        })
    }

    pub fn db(&self) -> Database {
        let db = self.db_pool.database("rust-crud");
        return db;
    }

    pub fn config(&self) -> &Args { &self.config }

    pub fn argon(&self) -> &Argon { &self.argon }
}

pub fn with_env(env: Environment) -> impl Filter<Extract=(Environment, ), Error=Infallible> + Clone {
    warp::any().map(move || env.clone())
}
