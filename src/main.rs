use std::{hash::{DefaultHasher, Hash, Hasher}, path::{Path, PathBuf}};

use anyhow::Context;
use av1an_core::{concat::ConcatMethod, context::Av1anContext, encoder::Encoder, ffmpeg, settings::{EncodeArgs, InputPixelFormat, PixelFormat}, Input};
use clap::Parser;
use cli::Cli;
use ffmpeg_the_third::format::Pixel;
use flexi_logger::{LogSpecBuilder, Logger};
use walkdir::WalkDir;

mod cli;

pub fn hash_path(path: &Path) -> String {
    let mut s = DefaultHasher::new();
    path.hash(&mut s);
    format!("{:x}", s.finish())[..7].to_string()
}

fn get_encode_args(cli: &Cli, input: &PathBuf, output: PathBuf) -> anyhow::Result<EncodeArgs> {
    let input = Input::Video(input.clone());
    let output_file = output
        .to_str()
        .expect("Failed to convert PathBuf to &str")
        .to_owned();

    let output_pix_format = PixelFormat {
        format: Pixel::YUV420P10LE,
        bit_depth: Encoder::svt_av1.get_format_bit_depth(Pixel::YUV420P10LE).expect("Failed to get bit depth")
    };

    let ffmpeg_filter_args = Vec::new();

    let video_params: Vec<String> = vec![
        "--preset", "6", "--tune", "3", "--crf", "23", "--lp", "2"
    ].into_iter().map(|f| f.to_owned()).collect();

    let audio_params: Vec<String> = vec![
        "-c:a libopus -b:a 128k"
    ].into_iter().map(|f| f.to_owned()).collect();

    let temp = format!(".{}", hash_path(input.as_path()));

    let encode_args = EncodeArgs {
        input: input.clone(),
        temp: temp.clone(),
        output_file,
        chunk_method: av1an_core::ChunkMethod::LSMASH,
        chunk_order: av1an_core::ChunkOrdering::LongestFirst,
        scaler: {
            let mut scaler = String::from("bicubic").clone();
            let mut scaler_ext = "+accurate_rnd+full_chroma_int+full_chroma_inp+bitexact".to_string();
            if scaler.starts_with("lanczos") {
              for n in 1..=9 {
                if scaler.ends_with(&n.to_string()) {
                  scaler_ext.push_str(&format!(":param0={}", &n.to_string()));
                  scaler = "lanczos".to_string();
                }
              }
            }
            scaler.push_str(&scaler_ext);
            scaler
          },
        scenes: None,
        split_method: av1an_core::SplitMethod::AvScenechange,
        sc_pix_format: None,
        sc_method: av1an_core::ScenecutMethod::Standard,
        sc_only: false,
        sc_downscale_height: Some(480),
        extra_splits_len: Some(240),
        min_scene_len: 240,
        force_keyframes: Vec::new(),
        ignore_frame_mismatch: false,
        max_tries: 3,
        passes: Encoder::svt_av1.get_default_pass(),
        video_params,
        encoder: Encoder::svt_av1,
        workers: 6,
        set_thread_affinity: Some(2),
        photon_noise: cli.ph,
        photon_noise_size: (None, None),
        chroma_noise: cli.chroma_noise,
        zones: None,
        ffmpeg_filter_args,
        audio_params,
        input_pix_format: {
            match &input {
                Input::Video(path) => InputPixelFormat::FFmpeg {
                    format: ffmpeg::get_pixel_format(path.as_ref()).with_context(|| {
                      format!("FFmpeg failed to get pixel format for input video {path:?}")
                    })?,
                },
                Input::VapourSynth(path) => InputPixelFormat::VapourSynth {
                    bit_depth: av1an_core::vapoursynth::bit_depth(path.as_ref()).with_context(|| {
                      format!("VapourSynth failed to get bit depth for input video {path:?}")
                    })?,
                },
            }
        },
        verbosity: av1an_core::Verbosity::Verbose,
        log_file: Path::new(&temp).join("log.log"),
        resume: true,
        keep: false,
        force: false,
        concat: ConcatMethod::MKVMerge,
        target_quality: None,
        output_pix_format,
    };

    Ok(encode_args)
}

fn get_output_path(src: &PathBuf, dst: &PathBuf, path: &PathBuf) -> PathBuf {
    let relative = path.strip_prefix(src).expect("Not a prefix");
    dst.join(relative)
}

fn encode_file(cli: &Cli, input: PathBuf) {
    let output = get_output_path(&cli.input, &cli.output, &input);
    let encode_args = get_encode_args(cli, &input, output).expect("Failed to get encode args");

    Av1anContext::new(encode_args)
        .expect("Failed to create av1an context")
        .encode_file()
        .expect("Failed to encode file");
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

    let _logger = Logger::with(log)
        .log_to_stderr()
        .start().expect("Failed to run logger");

    let files = WalkDir::new(&cli.input)
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
    
    for file in files {
        encode_file(&cli, file)
    }
}
