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
2.  **Build and run the application:**
    ```bash
    cargo run --release
    ```
    The `--release` flag is recommended for better performance.

## Usage

- **Launch the application:** Run the executable from your terminal or file explorer.
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
