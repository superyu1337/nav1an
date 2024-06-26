use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to a folder to encode
    #[arg(default_value = PathBuf::from("./input").into_os_string())]
    pub input: PathBuf,

    /// Path to a folder to encode
    #[arg(default_value = PathBuf::from("./output").into_os_string())]
    pub output: PathBuf,

    /// CRF value to use
    #[arg(short, long, default_value_t = 23)]
    pub crf: usize,

    /// SVT-AV1-PSY preset to use
    #[arg(short, long, default_value_t = 3)]
    pub preset: usize,

    /// SVT-AV1-PSY tune to use
    #[arg(short, long, default_value_t = 3)]
    pub tune: usize,

    /// Photon noise amount
    #[arg(long)]
    pub ph: Option<u8>,

    /// Photon noise amount
    #[arg(long, default_value_t = false)]
    pub chroma_noise: bool,

    /// Film grain amount
    #[arg(long)]
    pub fg: Option<usize>,

    /// Film grain denoising
    #[arg(long, default_value_t = false)]
    pub fgd: bool
}