# clack-rs

Clack-rs is a distraction-free terminal typewriter designed for focused writing. It aims to provide a tactile and immersive writing experience through unique features and custom sound design, mimicking the feel of a classic typewriter.

## project status

This project is currently in very early and active development. Features may change, and bugs are to be expected. Contributions and feedback are welcome as the project evolves.

## features

*   **distraction-free writing:** A clean interface focused solely on your text.
*   **typewriter mode:** Keeps the active line vertically centered on the screen, similar to a physical typewriter.
*   **focus mode:** Dims inactive lines, drawing attention to your current line of thought.
*   **theming:** Cycle through different visual themes (dark, paper, retro) to suit your preference.
*   **authentic sounds:** Mechanical keyboard sound effects for key presses, space, backspace, and a carriage return "thunk". Includes a classic end-of-line bell warning.
*   **fixed-width paper:** Simulates a physical sheet of paper with consistent margins, centered in your terminal.
*   **basic text editing:** Insert and delete characters, navigate left/right/up/down.
*   **markdown rendering:** Supports basic inline markdown for bold and italic text.
*   **file management:** Save and load text files.
*   **typing statistics:** Displays word and character counts in the footer.

## technology stack

Clack-rs is built with Rust, leveraging the following libraries:

*   **ratatui:** For building the terminal user interface.
*   **crossterm:** For cross-platform terminal event handling and manipulation.
*   **ropey:** An efficient text buffer (rope data structure) for robust text editing.
*   **rodio:** For audio playback of sound effects.
*   **anyhow:** For simplified error handling.

## installation and usage

To build and run clack-rs, you will need the Rust toolchain installed.

1.  **clone the repository:**
    ```bash
    git clone https://github.com/your-username/clack # (if you named your repo 'clack')
    cd clack
    ```

2.  **build and run:**
    ```bash
    cargo run
    ```
    Alternatively, to open or create a file:
    ```bash
    cargo run -- my_document.md
    ```

## keybindings

*   **escape:** Quit the application.
*   **F2:** Toggle focus mode (dims inactive lines).
*   **F3 / control + t:** Toggle typewriter mode (keeps active line centered).
*   **F4:** Toggle sound effects.
*   **F5:** Cycle through available themes (dark, paper, retro).
*   **control + s:** Save the current document.
*   **arrow keys:** Navigate characters and lines.
*   **backspace:** Delete previous character.
*   **enter:** Insert a new line.

## sound assets

For the full audio experience, ensure you have your custom sound files in the `src/assets/` directory:

*   `manual_key.wav` (for character input)
*   `manual_space.wav` (for spacebar)
*   `manual_backspace.wav` (for backspace)
*   `manual_return.wav` (for enter key)
*   `manual_bell.wav` (for end-of-line warning)
*   `manual_load_long.wav` (for application startup)
*   `manual_shift.wav` (for toggling features)

## inspiration

Clack-rs draws inspiration from various sources aiming to recreate a focused writing environment. A particular influence for the authentic sound design was the `typewriter-sounds` project:

*   [https://github.com/aizquier/typewriter-sounds](https://github.com/aizquier/typewriter-sounds)

---
clack-rs is licensed under [insert license here, e.g., MIT, Apache 2.0].
