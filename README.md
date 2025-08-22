## Please note

This app is vibe coded, and that goes for this README as well. I plan to clean this up in the future, but for now this information might be incorrect, the code might be error prone and you use the app at your own risk!

# Rustydoro

Rustydoro is a minimalist Pomodoro timer designed for focus. Instead of a traditional clock, it uses a subtle, full-window color transition to indicate the passage of time, creating a calm and unobtrusive user experience.

The timer is turqouise at idle and then progresses through a 30-minute cycle, changing from green to yellow, to red, and finally to black, signaling the end of the session when the window turns turqouise again.

## Features

- **Visual Timer:** The entire application window changes color to show time progression.
- **Minimalist Interface:** No distracting numbers or complex controls.
- **Simple Controls:**
    - **Left-click:** Start or pause the timer.
    - **Right-click:** Reset the timer.
- **Keyboard Shortcuts:**
    - `Ctrl+Alt+S`: Start the timer.
    - `Ctrl+Alt+P`: Pause the timer.
    - `Ctrl+Q`: Quit the application.
- **Time Display:** A small, unobtrusive digital timer shows the remaining time at the bottom of the window.

## Color Stages

The 30-minute Pomodoro session is divided into the following color phases:

1.  **0-15 minutes:** Forest Green to Vivid Yellow
2.  **15-25 minutes:** Vivid Yellow to Pure Red
3.  **25-30 minutes:** Pure Red to Black

When the timer is idle, paused, or complete, the window is a calm turquoise color.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo package manager.

### Building from Source

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/MrOnijohn/rustydoro.git
    cd rustydoro
    ```
2.  **Build the application:**
    ```bash
    cargo build --release
    ```
    The `--release` flag is recommended for better performance. The executable will be located at `target/release/rustydoro`.

### Desktop Integration (Linux)

For a more convenient desktop experience, you can create a `.desktop` file to launch Rustydoro from your application menu.

1.  **Install the executable:**
    Move the compiled binary to a directory in your `$PATH`. A common choice is `~/.local/bin`:
    ```bash
    mkdir -p ~/.local/bin
    cp target/release/rustydoro ~/.local/bin/
    ```

2.  **Create a `.desktop` file:**
    Create a file named `rustydoro.desktop` in `~/.local/share/applications/` with the following content:
    ```ini
    [Desktop Entry]
    Name=Rustydoro
    Comment=A minimalist Pomodoro timer
    Exec=rustydoro
    Icon=/home/john/Programmering/Rust/rustydoro/assets/icons/icon-256x256.png
    Terminal=false
    Type=Application
    Categories=Utility;
    ```
    *Note: Make sure the `Icon` path is correct. You may need to adjust it depending on where you cloned the repository.*

3.  **Update the desktop database:**
    Run the following command to register the new desktop entry:
    ```bash
    update-desktop-database ~/.local/share/applications
    ```

After these steps, you should be able to find and launch Rustydoro from your application menu.

## Usage

- **Launch the application:** Run the executable from your terminal, or from your desktop environment if you completed the desktop integration.
- I recommend putting the window to one side of the screen, making it as thin as needed/wanted, and doing work in a window next to it. The point is to not have to check a timer to see how long there is left of a focus session, but instead get an intuitive feel for how much time is left by having a color in the side of your vision.
- **Start the timer:** Left-click anywhere in the window or press `Ctrl+Alt+S`.
- **Pause the timer:** Left-click while the timer is running or press `Ctrl+Alt+P`.
- **Reset the timer:** Right-click anywhere in the window.
- **Quit:** Close the window or press `Ctrl+Q`.

## Customization

The timer duration and color scheme can be modified directly in the `src/main.rs` file.

- **Timer Duration:** Change the `total_duration` value in the `PomodoroApp::default()` function.
- **Colors:** Modify the RGB values in the `get_color` method to create your own custom theme.

## License

This project is open-source and available under the MIT License.