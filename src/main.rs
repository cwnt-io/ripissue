mod args;
mod elements;
mod helpers;
mod properties;
mod executors;

extern crate slugify;
use crate::args::{Cli,EntityType};
use clap::Parser;
use anyhow::Result;

use elements::issue::Issue;
use elements::sprint::Sprint;
use elements::elem::ElemBase;
use executors::run_subc::Runnable;


fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Issue(subc) => {
            let mut elem = Issue::new();
            elem.run_cmd(subc)?;
        }
        EntityType::Sprint(subc) => {
            let mut elem = Sprint::new();
            elem.run_cmd(subc)?;
        }
    }


        // EntityType::Issue(SubCommand::List(cmd)) => {
        //     let mut issues = Elems::new(ElemType::Issue);
        //     issues.update()?;
        //     let stdout = stdout();
        //     let mut writer = BufWriter::new(stdout);
        //     writeln!(writer,"{} #{} ({})",
        //     Issue::elem().to_uppercase(),
        //     &id,
        //     issue.display()
        //     )?;
        //     let issues = get_all_elems::<Issue>()?;
        //     println!("{:#?}", issues);
        // }

    Ok(())
}
