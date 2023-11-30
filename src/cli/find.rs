use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::error::Error;

use super::subcommand::SubcommandHandler;
use crate::utils::paths::filtered_search;
use crate::utils::paths::{app_root, to_short_path};

#[derive(Parser, Debug)]
pub struct FindArgs {
    store_name: String,
}

pub struct FindHandler {
    store_name: String,
}

impl FindHandler {
    pub fn new(args: FindArgs) -> Self {
        FindHandler {
            store_name: args.store_name.to_owned(),
        }
    }
}

#[async_trait]
impl SubcommandHandler for FindHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let store_paths: Vec<_> = filtered_search(&app_root(), &move |path| {
            path.is_dir() && path.display().to_string().contains(&self.store_name)
        })
        .unwrap_or(Vec::new())
        .into_par_iter()
        .filter_map(|x| to_short_path(&x).ok())
        .collect();

        if store_paths.is_empty() {
            Ok(Some("Nothing found".to_string()))
        } else {
            store_paths.into_par_iter().for_each(|x| {
                println!("{}", x.display().to_string().cyan());
            });

            Ok(None)
        }
    }
}
