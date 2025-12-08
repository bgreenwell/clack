# AGENTS.md

## 1. Project Overview

**Clack** is a Rust-based, distraction-free terminal typewriter application. It is designed to emulate the tactile and auditory experience of a physical typewriter within a modern CLI environment.

### Key Features
-   **Typewriter Mode:** Maintains the active line at the vertical center of the screen.
-   **Audio Engine:** Low-latency playback of mechanical sounds (keystrokes, bell, carriage return) using `rodio`.
-   **Text Engine:** Efficient text manipulation using `ropey` (Rope data structure).
-   **UI/UX:** A terminal interface built with `ratatui` and `crossterm`, supporting theming and markdown rendering.

## 2. Rust Development Standards

All changes must strictly adhere to the following Rust conventions and workflows.

### Essential Commands
Agents must run these commands to verify code quality before finishing a task.

1.  **Format:** `cargo fmt`
    *   **Rule:** Code must always be formatted with the default Rust formatter.
2.  **Lint:** `cargo clippy -- -D warnings`
    *   **Rule:** Address all clippy warnings. Treat them as errors.
3.  **Check:** `cargo check`
    *   **Rule:** Run frequently during development to catch type errors early without full compilation.
4.  **Test:** `cargo test`
    *   **Rule:** Ensure all unit and integration tests pass. Add new tests for new logic.

### Coding Conventions
-   **Error Handling:**
    *   Use `anyhow::Result` for application-level error handling (e.g., in `main.rs` or high-level controllers).
    *   Use `thiserror` or standard typed errors for library-level modules if applicable.
    *   **Avoid `unwrap()` or `expect()`** in production code unless verifying a guaranteed invariant (comment required).
-   **State Management:**
    *   The `App` struct (`src/app.rs`) is the single source of truth.
    *   Avoid global mutable state.
-   **Concurrency:**
    *   Audio is handled in a separate thread (`src/sound.rs`). Ensure strict thread safety when modifying audio triggers.
-   **Dependencies:**
    *   Do not add new dependencies without explicit reasoning and user confirmation.
    *   Stick to `ratatui` for UI and `crossterm` for events.

## 3. Git & Version Control Practices

Maintain a clean, linear, and meaningful history.

### Commit Messages
Use the **Conventional Commits** format:
-   `feat: ...` for new features.
-   `fix: ...` for bug fixes.
-   `refactor: ...` for code restructuring without behavioral change.
-   `style: ...` for formatting or missing semi-colons.
-   `docs: ...` for documentation updates.

**Example:** `feat: add toggle for typewriter mode in app state`

### Workflow Rules
-   **Atomic Commits:** Make small, self-contained changes. A commit should be compilable and passable on its own.
-   **Verification:** Run `cargo check` and `cargo test` *before* creating a commit.
-   **No Broken Code:** Never commit code that fails to compile.

## 4. Architecture & File Structure

*   **`src/main.rs`**: Application entry point. Handles terminal setup/teardown and the main event loop.
*   **`src/app.rs`**: Core logic. Contains the `App` struct, state definitions (cursor, mode, content), and state mutation methods.
*   **src/ui.rs**: View layer. Pure function of `App` state. Renders widgets using `ratatui`.
*   **src/sound.rs**: Audio subsystem. Spawns a background thread for non-blocking sound playback.
*   **src/theme.rs**: Visual definitions for colors and styles.

## 5. Agent Interaction Protocol

1.  **Read First:** Always read `AGENTS.md` (this file) and the relevant source files before planning changes.
2.  **Codebase Investigator:** Use the investigator tool for broad architectural questions.
3.  **Tests as Spec:** When fixing bugs, create a reproduction test case first if possible.
4.  **No Fluff:** Do not modify `AGENTS.md` unless specifically instructed to update project standards.