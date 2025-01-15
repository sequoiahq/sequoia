use std::error::Error as StdError;
use std::process::Command;

pub fn download_video(
    m3u8_url: &str,
    filename: &str,
    quality: Option<&str>,
) -> Result<(), Box<dyn StdError>> {
    let mut command = vec![
        "N_m3u8DL-RE",
        m3u8_url,
        "--save-name",
        filename,
        "--thread-count",
        "40",
        "--mux-after-done",
        "mkv",
        "--auto-select",
    ];

    if let Some(q) = quality {
        command.push("--select-video");
        command.push(q);
    }

    let status = Command::new(command[0]).args(&command[1..]).status()?;

    if !status.success() {
        Err("Download failed")?
    }

    Ok(())
}
