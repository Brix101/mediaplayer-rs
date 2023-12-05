use mediaplayer::cmd::Args;

fn main() {
    let args = Args::parse();

    // Access the parsed arguments
    println!("Hello {}", args.player.unwrap_or("world".to_string()));
}
