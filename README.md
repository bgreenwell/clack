# clack-rs

Clack-rs is a distraction-free terminal typewriter designed for fun and focused writing. It aims to provide a tactile and immersive writing experience through unique features and custom sound design, mimicking the feel of a classic typewriter.

## Warning

This project is currently in very early and active development. Features may change, and bugs are to be expected. Contributions and feedback are welcome as the project evolves.

## Features

*   **Distraction-free writing:** A clean interface focused solely on your text.
*   **Typewriter mode:** Keeps the active line vertically centered on the screen, similar to a physical typewriter.
*   **Focus mode:** Dims inactive lines, drawing attention to your current line of thought.
*   **Theming:** Cycle through different visual themes (dark, paper, retro) to suit your preference.
*   **Authentic sounds:** Mechanical keyboard sound effects for key presses, space, backspace, and a carriage return "thunk". Includes a classic end-of-line bell warning.
*   **Fixed-width paper:** Simulates a physical sheet of paper with consistent margins, centered in your terminal.
*   **Basic text editing:** Insert and delete characters, navigate left/right/up/down.
*   **Markdown rendering:** Supports (VERY) basic inline markdown for bold and italic text.
*   **File management:** Save and load text files.
*   **Typing statistics:** Displays word and character counts in the footer.

## Technology stack

Clack-rs is built with Rust, leveraging the following libraries:

*   **ratatui:** For building the terminal user interface.
*   **crossterm:** For cross-platform terminal event handling and manipulation.
*   **ropey:** An efficient text buffer (rope data structure) for robust text editing.
*   **rodio:** For audio playback of sound effects.
*   **anyhow:** For simplified error handling.

## Installation and usage

To build and run clack-rs, you will need the Rust toolchain installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/clack # (if you named your repo 'clack')
    cd clack
    ```

2.  **Build and run:**
    ```bash
    cargo run
    ```
    Alternatively, to open or create a file:
    ```bash
    cargo run -- my_document.md
    ```

## keybindings

*   **Escape:** Quit the application.
*   **F2:** Toggle focus mode (dims inactive lines).
*   **F3 / control + t:** Toggle typewriter mode (keeps active line centered).
*   **F4:** Toggle sound effects.
*   **F5:** Cycle through available themes (dark, paper, retro).
*   **Ctrl + s:** Save the current document.
*   **Arrow keys:** Navigate characters and lines.
*   **Backspace:** Delete previous character.
*   **Enter:** Insert a new line.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
