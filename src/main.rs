use crossterm::{
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{self, KeyCode, KeyEvent},
    cursor,
};
use rodio::{Decoder, OutputStream, source::Source};
use std::io::{self, Write, BufReader};
use std::time::Duration;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut stdout = io::stdout();
    let mut files = Vec::new();
    let mut selected_index = 0;
    let mut error_message = String::new();

    // Enter alternate screen
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    enable_raw_mode()?;

    // Load the files in the current directory
    let paths = std::fs::read_dir(".").unwrap();
    for path in paths {
        files.push(path.unwrap().path());
    }

    loop {
        // Clear the screen
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Display the list of files with the selected one highlighted
        for (i, file) in files.iter().enumerate() {
            if i == selected_index {
                print!("\r> {}", file.display());
                if !error_message.is_empty() {
                    print!(" - {}", error_message);
                }
                println!();
            } else {
                println!("\r  {}", file.display());
            }
        }

        stdout.flush()?;

        // Poll for user input
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                error_message.clear();
                match code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < files.len().saturating_sub(1) {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let file_path = files[selected_index].clone();
                        match File::open(&file_path) {
                            Ok(file) => {
                                let source = Decoder::new(BufReader::new(file));
                                match source {
                                    Ok(decoded) => {
                                        stream_handle.play_raw(decoded.convert_samples()).ok();
                                    }
                                    Err(_) => {
                                        error_message = "File is not a recognized audio format".to_string();
                                    }
                                }
                            }
                            Err(_) => {
                                error_message = "Unable to open file".to_string();
                            }
                        }
                    }
                    KeyCode::Esc => break, // Exit on Esc key
                    KeyCode::Char('q') => break, // Exit on 'q'
                    _ => {}
                }
            }
        }
    }

    // Exit alternate screen and flush output
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    stdout.flush()?;

    Ok(())
}

