mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use crate::args::Cli;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.entity_type.run_cmd()?;

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
