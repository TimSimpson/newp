use std::path::PathBuf;

use structopt::StructOpt;
use crate::templates;


#[derive(Debug, StructOpt)]
#[structopt(name = "newp", about = "Creates a new project")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,    
}


#[derive(Debug,StructOpt)]
enum Command {
    Create(CreateArgs),
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "command" )]
struct CreateArgs {
    #[structopt(short, long, about="type of new project")]
    template: String,
    #[structopt(short, long, about="name of new project")]
    name: String,
    #[structopt(long, about="description or summary of what this project does")]
    description: String,
    #[structopt(long, about="directory (defaults to a new directory named after this project")]
    directory: Option<String>,
}

pub fn start() {
    let opt = Opt::from_args();
    match opt.command {
        Command::List => {
            show_list();
        },
        Command::Create(args) => {
            let directory = match args.directory {
                Some(p) => p.into(),
                None => (args.name.clone()).into(),
            };
            create(templates::RenderOptions { description: args.description, name: args.name, r#type: args.template }, directory);
        }
    }
}

fn create(options: templates::RenderOptions, directory: PathBuf) {
    let parent_exists = match directory.parent() {
        None => false,
        Some(p) => p.exists()
    };
    if  !parent_exists {
        eprintln!("Could not find parent directory for {:?}.", directory);
        std::process::exit(0);
    }

    let content = templates::render(&options);
    for (file, content) in content.into_iter() {
        let mut full_path = directory.clone();
        full_path.push(file);
        if let Err(e) = std::fs::create_dir_all(full_path.parent().unwrap()) {
            eprintln!("Error creating directory: {:?}: {}", full_path.parent(), e);
            std::process::exit(0);
        }
        if let Err(e) = std::fs::write(&full_path, content) {
            eprintln!("Error writing file {:?}: {}", full_path, e);
        }
    }

}

fn show_list() {
    let list = templates::get_list();
    for item in list {
        println!("{}", item.name);
        println!("\t{}", item.desc);
        println!();
    }
}
