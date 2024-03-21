use std::fs;

use av1an_core::{context::Av1anContext, settings::EncodeArgs};
use clap::Parser;
use cli::Cli;
use flexi_logger::{LogSpecBuilder, Logger};
use walkdir::WalkDir;

mod cli;

fn get_encode_args(args: Cli) -> anyhow::Result<EncodeArgs> {
    let encode_args = EncodeArgs {
        input: todo!(),
        temp: todo!(),
        output_file: todo!(),
        chunk_method: todo!(),
        chunk_order: todo!(),
        scaler: todo!(),
        scenes: todo!(),
        split_method: todo!(),
        sc_pix_format: todo!(),
        sc_method: todo!(),
        sc_only: todo!(),
        sc_downscale_height: todo!(),
        extra_splits_len: todo!(),
        min_scene_len: todo!(),
        force_keyframes: todo!(),
        ignore_frame_mismatch: todo!(),
        max_tries: todo!(),
        passes: todo!(),
        video_params: todo!(),
        encoder: todo!(),
        workers: todo!(),
        set_thread_affinity: todo!(),
        photon_noise: todo!(),
        photon_noise_size: todo!(),
        chroma_noise: todo!(),
        zones: todo!(),
        ffmpeg_filter_args: todo!(),
        audio_params: todo!(),
        input_pix_format: todo!(),
        output_pix_format: todo!(),
        verbosity: todo!(),
        log_file: todo!(),
        resume: todo!(),
        keep: todo!(),
        force: todo!(),
        concat: todo!(),
        target_quality: todo!(),
    };

    Ok(encode_args)
}

fn main() {
    let cli = Cli::parse();

    let log = LogSpecBuilder::new()
        .default(log::LevelFilter::Error)
        .module("av1an", log::LevelFilter::Error)
        .module("rav1an", log::LevelFilter::Error)
        .module("av1an_core", log::LevelFilter::Error)
        .module("rav1e::scenechange", log::LevelFilter::Error)
        .build();

    let logger = Logger::with(log)
        .log_to_stderr()
        .start().expect("Failed to run logger");


    let files = WalkDir::new(cli.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .fold(Vec::new(), |mut vec, entry| {
            let fname = entry.file_name().to_string_lossy();

            if fname.ends_with(".mkv") || fname.ends_with(".mp4") {
                vec.push(entry.path().to_path_buf())
            }
            
            vec
        });
    
    println!("{:#?}", files);
}
