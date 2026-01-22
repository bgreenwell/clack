# Clack

Clack is a distraction-free terminal typewriter designed for fun and focused writing. It aims to provide a tactile and immersive writing experience through unique features and custom sound design, mimicking the feel of a classic typewriter.

![Clack Demo](demo.gif)

## Warning

This project is currently in very early and active development. Features may change, and bugs are to be expected. Contributions and feedback are welcome as the project evolves.

## Features

*   **Distraction-free writing:** A clean interface focused solely on your text.
*   **Typewriter mode:** Keeps the active line vertically centered on the screen, similar to a physical typewriter.
*   **Focus mode:** Dims inactive lines, drawing attention to your current line of thought.
*   **Theming:** Cycle through different visual themes (dark, paper, retro) to suit your preference.
*   **Authentic sounds:** Mechanical keyboard sound effects for key presses, space, backspace, and a carriage return "thunk". Includes a classic end-of-line bell warning at 72 characters.
*   **Fixed-width paper:** Simulates a physical sheet of paper with consistent margins, centered in your terminal.
*   **Margin guide:** Subtle visual indicator at column 72 to help you stay within typewriter margins.
*   **Advanced navigation:** Word-wise movement (Ctrl+Arrow), Home/End keys, and Delete key support.
*   **Markdown rendering:** Supports basic inline markdown for bold and italic text.
*   **File management:** Save and load text files with unsaved changes indicator.
*   **Typing statistics:** Displays word and character counts in the footer.
*   **Status feedback:** Visual confirmation for save operations and clear error messages.

## Technology stack

Clack is built with Rust, leveraging the following libraries:

*   **ratatui:** For building the terminal user interface.
*   **crossterm:** For cross-platform terminal event handling and manipulation.
*   **ropey:** An efficient text buffer (rope data structure) for robust text editing.
*   **rodio:** For audio playback of sound effects.
*   **anyhow:** For simplified error handling.

## Installation and usage

### From source

To build and run Clack from source, you will need the Rust toolchain installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/clack
    cd clack
    ```

2.  **Build and run:**
    ```bash
    cargo run --release
    ```
    Or install locally:
    ```bash
    cargo install --path .
    clack my_document.md
    ```

3.  **Usage:**
    ```bash
    # Start with a new document
    clack

    # Open or create a specific file
    clack my_document.md
    ```

## Recommended fonts for maximum typewriter feel

While Clack can't control your terminal's font directly, using a typewriter-style monospace font will dramatically enhance the experience. Here are our top recommendations:

### Best Typewriter Fonts

1. **IBM Plex Mono** (Free) - Modern take on IBM's classic typewriter font
   - Download: https://fonts.google.com/specimen/IBM+Plex+Mono
   - Best weight: Regular or Medium

2. **Courier Prime** (Free) - Designed specifically to look like a typewriter
   - Download: https://quoteunquoteapps.com/courierprime/
   - Most authentic typewriter feel

3. **JetBrains Mono** (Free) - Clean, modern monospace with excellent readability
   - Download: https://www.jetbrains.com/lp/mono/
   - Good for extended writing sessions

4. **Inconsolata** (Free) - Classic programmer font with typewriter aesthetics
   - Download: https://fonts.google.com/specimen/Inconsolata

5. **American Typewriter** (macOS built-in) - Authentic typewriter appearance
   - No download needed on macOS

### How to change your terminal font

- **macOS Terminal**: Terminal → Preferences → Profiles → Font
- **iTerm2**: Preferences → Profiles → Text → Font
- **Alacritty**: Edit `~/.config/alacritty/alacritty.yml`
- **Windows Terminal**: Settings → Profiles → Appearance → Font face

### Recommended settings

For the best typewriter experience:
- **Font size**: 13-16pt (larger is more typewriter-like)
- **Line spacing**: 1.2-1.4 (gives that classic typewritten look)
- **Theme**: Use Clack's "Paper" theme (F5) with a cream-colored font for authenticity

## Keybindings

### Quick reference
*   **F1:** Show help menu with all keyboard shortcuts
*   **Ctrl + S:** Save the current document
*   **Escape:** Quit the application

### Application controls (see F1 for full list)
*   **F2:** Toggle focus mode (dims inactive lines)
*   **F3 / Ctrl + T:** Toggle typewriter mode (keeps active line centered)
*   **F4:** Toggle sound effects
*   **F5:** Cycle through available themes (Dark, Paper, Retro)
*   **F6:** Toggle double spacing

### Text Editing
*   **Backspace:** Delete previous character.
*   **Delete:** Delete next character (forward delete).
*   **Enter:** Insert a new line.

### Navigation
*   **Arrow keys:** Navigate characters and lines.
*   **Ctrl + Left/Right:** Jump by word.
*   **Home:** Move to beginning of line.
*   **End:** Move to end of line.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
