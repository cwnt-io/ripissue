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

    println!("Enterred here");
    println!("{:?}", &cli.entity_type.to_string());
    cli.entity_type.run_cmd()?;

    // match &cli.entity_type {
    //     EntityType::Issue(subc) => {
    //         // let mut elem = Issue::new();
    //         // elem.run_cmd(subc)?;
    //     }
    //     EntityType::Sprint(subc) => {
    //         // let mut elem = Sprint::new();
    //         // elem.run_cmd(subc)?;
    //     }
    // }


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
