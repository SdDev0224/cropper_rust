use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CliOpts {
    // This path is origin image ( Source Image )
    #[arg(
        short = 's',
        long,
        value_name = "SOURCES",
        default_value = "./thisimages"
    )]
    pub source_image_path : String,
    // This is processed image path
    #[arg(
        short = 'p',
        long,
        value_name = "PROCESSED",
        default_value = "./processedimages"
    )]
    pub processed_image_path : String,
    // This is result images path
    #[arg(
        short = 'r',
        long,
        value_name = "RESULTS",
        default_value = "./resultsimages"
    )]
    pub result_image_path : String,
    // This is left and top
    #[arg(
        short = 't',
        long,
        value_name = "LEFTTOP",
        default_value = "0,0"
    )]
    pub left_top : String,
    // This is  right and button
    #[arg(
        short = 'b',
        long,
        value_name = "RIGHTBOTTOM",
        default_value = "20,15"
    )]
    pub right_bottom : String,
    // This is Tag name 
    #[arg(
        short = 'w',
        long,
        value_name = "TAG-NAME",
        default_value = "center"
    )] 
    pub tag_name : String,
}

impl CliOpts {
    pub fn parse_cli() -> Self {
        CliOpts::parse()
    }
}
