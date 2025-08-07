mod debruijn;
mod pinwheel;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tiling-gallery")]
#[command(about = "generate tiling image", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成彭罗斯镶嵌
    Debruijn {
        #[arg(short, long, default_value_t = 5)]
        dimension: usize,
        #[arg(short, long, default_value_t = 12)]
        num_lines: i32,
        #[arg(short, long, default_value_t = 800)]
        width: u32,
        #[arg(short, long, default_value_t = 600)]
        height: u32,
        #[arg(short, long, default_value = "#FF9F1C")]
        fat_color: String,
        #[arg(short, long, default_value = "#2EC4B6")]
        thin_color: String,
        #[arg(short, long, default_value = "#6D6875")]
        edge_color: String,
        #[arg(short, long, default_value = "output.svg")]
        output_filename: String,
    },
    /// 生成pinwheel镶嵌
    Pinwheel {
        #[arg(short, long, default_value_t = 5)]
        iterations: u32,
        #[arg(short, long, default_value_t = 800)]
        width: u32,
        #[arg(short, long, default_value_t = 600)]
        height: u32,
        #[arg(short, long, default_value = "output.svg")]
        output_filename: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Debruijn {
            dimension,
            num_lines,
            width,
            height,
            fat_color,
            thin_color,
            edge_color,
            output_filename,
        } => {
            debruijn::generate(
                dimension,
                num_lines,
                width,
                height,
                fat_color,
                thin_color,
                edge_color,
                &output_filename,
            )?;
            println!("generate svg to {}", output_filename);
            Ok(())
        }
        Commands::Pinwheel {
            iterations,
            width,
            height,
            output_filename,
        } => {
            pinwheel::generate(iterations, width, height, &output_filename);
            println!("generate svg to {}", output_filename);
            Ok(())
        }
    }
}
