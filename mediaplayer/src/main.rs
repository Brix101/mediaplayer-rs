use std::process;

use mediaplayer::cmd::Args;
use playerctl::PlayerManager;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let player = PlayerManager::new(args.player);
    player.run();

    // Run the 'playerctl -l' command
    let output = process::Command::new("playerctl")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("{:#?}", output);
    println!("{:?}", String::from_utf8_lossy(&output.stderr));
    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a UTF-8 string
        let result = String::from_utf8_lossy(&output.stdout);

        // Print the result
        println!("Playerctl -l result: {}", result);
    } else {
        // Print the error message
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
