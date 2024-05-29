use config::CliOpts;
use futures::future::join_all;
use image::{self, GenericImageView, RgbaImage};
use lazy_static::lazy_static;
use log::info;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tokio::task;

mod config;

lazy_static! {
    static ref CLI_OPTS : CliOpts = CliOpts::parse_cli();
    static ref SOURCE_PATH : String = CLI_OPTS.source_image_path.clone();
    static ref PROCESS_PATH  : String = CLI_OPTS.processed_image_path.clone();
    static ref RESULT_PATH : String = CLI_OPTS.result_image_path.clone();
    static ref LEFT_TOP_VAL : String = CLI_OPTS.left_top.clone();
    static ref RIGHT_BOTTOM_VAL : String = CLI_OPTS.right_bottom.clone();
    static ref TAG_WORD : String = CLI_OPTS.tag_name.clone();
    // static ref root : PathBuf = env::current_dir().unwrap().parent().unwrap();
}

#[derive(Debug)]
pub enum CheckResult {
    IsFile,
    IsDir,
}

/**
 * Check file or directory exist
 */
async fn check_file(source_paths: &str) -> Result<CheckResult, io::Error> {
    let target_dir = Path::new("./")
        .join(Path::new("src"))
        .join(Path::new(source_paths));
    if target_dir.is_file() {
        Ok(CheckResult::IsFile)
    } else if target_dir.is_dir() {
        Ok(CheckResult::IsDir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File or directory not found",
        ))
    }
}

fn string_to_tuples(s: &str) -> (u32, u32) {
    let coords: Vec<&str> = s.split(',').collect();
    let x = coords[0].trim().parse::<u32>().unwrap();
    let y = coords[1].trim().parse::<u32>().unwrap();
    (x, y)
}

async fn crop_image(
    image_path: &str,
    left_top: (u32, u32),
    right_bottom: (u32, u32),
    output_path: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the image
    let img = image::open(image_path)?;

    // Ensure the coordinates are within the image dimensions
    let (img_width, img_height) = img.dimensions();
    if left_top.0 >= img_width || left_top.1 >= img_height {
        return Err("Left-top coordinates are out of image bounds".into());
    }
    if right_bottom.0 > img_width || right_bottom.1 > img_height {
        return Err("Right-bottom coordinates are out of image bounds".into());
    }
    if right_bottom.0 <= left_top.0 || right_bottom.1 <= left_top.1 {
        return Err("Right-bottom coordinates should be greater than left-top coordinates".into());
    }

    // Calculate width and height from coordinates
    let width = right_bottom.0 - left_top.0;
    let height = right_bottom.1 - left_top.1;

    // Crop the image
    let cropped_img: RgbaImage = img.view(left_top.0, left_top.1, width, height).to_image();

    // Save the cropped image

    let (base, extension) = file_name.split_at(file_name.rfind('.').unwrap());
    let new_file_name = format!("{}_{}.{}", base, TAG_WORD.clone(), extension);
    let result_path = Path::new(output_path).join(new_file_name);
    cropped_img.save(result_path)?;

    let processes_path = PROCESS_PATH.clone();
    let target_process = Path::new("./")
        .join(Path::new("src"))
        .join(Path::new(&processes_path))
        .join(file_name);
    img.save(target_process)?;
    if fs::metadata(image_path).is_ok() {
        fs::remove_file(image_path).expect("Failed to delete the image file");
    } else {
        eprintln!("The image file does not exist: {}", image_path);
    }
    Ok(())
}

async fn read_directory(source_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = Path::new("./")
        .join(Path::new("src"))
        .join(Path::new(source_folder));
    let paths: Vec<PathBuf> = fs::read_dir(target_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Collect all futures to join them later
    let futures: Vec<_> = paths
        .into_iter()
        .map(|path| {
            task::spawn(async move {
                if path.is_file() {
                    let file_path = path.to_str().unwrap();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let left_top = string_to_tuples(&LEFT_TOP_VAL);
                    let right_bottom = string_to_tuples(&RIGHT_BOTTOM_VAL);
                    let out_path_target = Path::new("./")
                        .join(Path::new("src"))
                        .join(Path::new(&RESULT_PATH.clone()));
                    let out_path_result = out_path_target.to_str().unwrap();
                    match crop_image(
                        file_path,
                        left_top,
                        right_bottom,
                        &out_path_result,
                        &file_name,
                    )
                    .await
                    {
                        Ok(_) => println!("Image cropped and saved successfully."),
                        Err(e) => eprintln!("Error cropping image: {}", e),
                    }
                }
            })
        })
        .collect();

    // Await all tasks
    join_all(futures).await;

    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .max_blocking_threads(1)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        // let start = Instant::now();
        info!("Starting the program...");
        match check_file(&SOURCE_PATH).await {
            Ok(CheckResult::IsDir) => {
                match read_directory(&SOURCE_PATH).await {
                    Ok(()) => panic!("hhh"),
                    Err(e) => print!("I am error {:?}", e),
                }
                // exit(1)
            }
            Ok(CheckResult::IsFile) => {
                let basic_path = SOURCE_PATH.clone();
                let target_dir = Path::new("./")
                    .join(Path::new("src"))
                    .join(Path::new(&basic_path));
                let finally_path = target_dir.to_str().unwrap();
                let left_top = string_to_tuples(&LEFT_TOP_VAL);
                let right_bottom = string_to_tuples(&RIGHT_BOTTOM_VAL);
                let file_name = target_dir.file_name().unwrap().to_str().unwrap();
                // let output_path =
                let out_path_target = Path::new("./")
                    .join(Path::new("src"))
                    .join(Path::new(&RESULT_PATH.clone()));
                let out_path_result = out_path_target.to_str().unwrap();
                match crop_image(
                    &finally_path,
                    left_top,
                    right_bottom,
                    out_path_result,
                    &file_name,
                )
                .await
                {
                    Ok(_) => println!("Image cropped and saved successfully."),
                    Err(e) => eprintln!("Error cropping image in a file: {}", e),
                }
                // exit(1)
            }
            Err(e) => {
                eprintln!("Error I can't file anything : {:?}", e)
            }
        }
    })
}
