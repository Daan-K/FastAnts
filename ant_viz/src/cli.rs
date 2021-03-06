#[derive(StructOpt, Debug)]
#[structopt(name = "ant_viz", about = "A visualizer for the ant language used in 2004's edition of the ICFP programming contest")]
pub struct Options {
    #[structopt(long = "world", help = "The path to the world file")]
    pub world: Option<String>,
    #[structopt(long = "red", help = "The path to the instructions of the red team")]
    pub red: Option<String>,
    #[structopt(long = "black", help = "The path to the instructions of the black team")]
    pub black: Option<String>,
    #[structopt(long = "rounds", help = "The amount of rounds to be executed", default_value = "100000")]
    pub rounds: u32,
    #[structopt(long = "seed", help = "The seed that will be used by the RNG", default_value = "12345")]
    pub seed: u32,
    #[structopt(long = "rps", help = "The amount of rounds that will be executed per second", default_value = "200")]
    pub rounds_per_second: u32,
}
