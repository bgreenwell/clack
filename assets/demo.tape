# VHS tape file for Clack demo
# Generate with: vhs demo.tape

Output demo.gif

Set Shell "bash"
Set FontSize 16
Set FontFamily "Courier Prime"
Set Width 1200
Set Height 800
Set TypingSpeed 100ms
Set PlaybackSpeed 1.0

# Build first
Type "cargo build --release"
Sleep 500ms
Enter
Sleep 8s

# Launch Clack
Type "./target/release/clack"
Sleep 300ms
Enter
Sleep 2s

# Type a sample text to show the typewriter in action
Type "The quick brown fox jumps over the lazy dog."
Sleep 800ms
Enter
Enter
Sleep 500ms

Type "Clack brings the tactile experience of a mechanical typewriter"
Sleep 300ms
Enter
Sleep 300ms

Type "to your terminal. Write distraction-free with centered text,"
Sleep 300ms
Enter
Sleep 300ms

Type "authentic sounds, and beautiful themes."
Sleep 1.5s
Enter
Enter
Sleep 800ms

Type "Happy writing!"
Sleep 2s

# Quit
Ctrl+Q
Sleep 1s
