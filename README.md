# clack-rs

Clack-rs is a distraction-free terminal typewriter designed for focused writing. It aims to provide a tactile and immersive writing experience through unique features and custom sound design, mimicking the feel of a classic typewriter.

## project status

This project is currently in very early and active development. Features may change, and bugs are to be expected. Contributions and feedback are welcome as the project evolves.

## features

*   **D**istraction-free writing: A clean interface focused solely on your text.
*   **T**ypewriter mode: Keeps the active line vertically centered on the screen, similar to a physical typewriter.
*   **F**ocus mode: Dims inactive lines, drawing attention to your current line of thought.
*   **T**heming: Cycle through different visual themes (dark, paper, retro) to suit your preference.
*   **A**uthentic sounds: Mechanical keyboard sound effects for key presses, space, backspace, and a carriage return "thunk". Includes a classic end-of-line bell warning.
*   **F**ixed-width paper: Simulates a physical sheet of paper with consistent margins, centered in your terminal.
*   **B**asic text editing: Insert and delete characters, navigate left/right/up/down.
*   **M**arkdown rendering: Supports basic inline markdown for bold and italic text.
*   **F**ile management: Save and load text files.
*   **T**yping statistics: Displays word and character counts in the footer.

## technology stack

Clack-rs is built with Rust, leveraging the following libraries:

*   **R**atatui: For building the terminal user interface.
*   **C**rossterm: For cross-platform terminal event handling and manipulation.
*   **R**opey: An efficient text buffer (rope data structure) for robust text editing.
*   **R**odio: For audio playback of sound effects.
*   **A**nyhow: For simplified error handling.

## installation and usage

To build and run clack-rs, you will need the Rust toolchain installed.

1.  **C**lone the repository:
    ```bash
    git clone https://github.com/your-username/clack # (if you named your repo 'clack')
    cd clack
    ```

2.  **B**uild and run:
    ```bash
    cargo run
    ```
    Alternatively, to open or create a file:
    ```bash
    cargo run -- my_document.md
    ```

## keybindings

*   **E**scape: Quit the application.
*   **F2**: Toggle focus mode (dims inactive lines).
*   **F3 / C**ontrol + t: Toggle typewriter mode (keeps active line centered).
*   **F4**: Toggle sound effects.
*   **F5**: Cycle through available themes (dark, paper, retro).
*   **C**ontrol + s: Save the current document.
*   **A**rrow keys: Navigate characters and lines.
*   **B**ackspace: Delete previous character.
*   **E**nter: Insert a new line.

## sound assets

For the full audio experience, ensure you have your custom sound files in the `src/assets/` directory:

*   `manual_key.wav` (**f**or character input)
*   `manual_space.wav` (**f**or spacebar)
*   `manual_backspace.wav` (**f**or backspace)
*   `manual_return.wav` (**f**or enter key)
*   `manual_bell.wav` (**f**or end-of-line warning)
*   `manual_load_long.wav` (**f**or application startup)
*   `manual_shift.wav` (**f**or toggling features)

## inspiration

Clack-rs draws inspiration from various sources aiming to recreate a focused writing environment. A particular influence for the authentic sound design was the `typewriter-sounds` project:

*   [https://github.com/aizquier/typewriter-sounds](https://github.com/aizquier/typewriter-sounds)

---
clack-rs is licensed under [insert license here, e.g., MIT, Apache 2.0].
