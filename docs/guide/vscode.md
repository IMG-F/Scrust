# VS Code Extension

Scrust provides a Visual Studio Code extension that offers rich syntax highlighting to enhance your development experience.

## Features

- **Syntax Highlighting**: Color-coded syntax that matches Scratch's block colors.
- **Semantic Highlighting**: Automatically identifies custom procedure calls and highlights them in Scratch's "My Blocks" pink color.
- **Extension Support**: Highlights supported extension blocks (e.g., Pen, Music) with their respective category colors.

## Color Mapping

The extension automatically applies "Scrust Block Colors" to your code, regardless of which VS Code theme you are using. This ensures that code visually resembles Scratch blocks.

- **Motion**: Blue (`move_steps`, `turn_right`, etc.)
- **Looks**: Purple (`say`, `switch_costume`, etc.)
- **Sound**: Magenta (`play_sound`, etc.)
- **Events**: Yellow (`#[on_flag_clicked]`, etc.)
- **Control**: Gold/Orange (`if`, `repeat`, `wait`)
- **Sensing**: Cyan (`touching`, `timer`, etc.)
- **Operators**: Green (`+`, `-`, `join`, etc.)
- **Variables**: Orange (`var`, `list`)
- **My Blocks (Procedures)**: Pink (`proc` definitions and calls)
- **Pen Extension**: Dark Green
- **Music Extension**: Dark Green

## Installation

### Building from Source

Currently, the extension is not yet published to the VS Code Marketplace, so you need to build it from source.

1. **Prerequisites**:
   - [Node.js](https://nodejs.org/) (v16 or higher)
   - [Git](https://git-scm.com/)

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/DilemmaGX/Scrust.git
   cd Scrust/editors/vscode
   ```

3. **Install Dependencies**:
   ```bash
   npm install
   ```

4. **Package the Extension**:
   The project includes a script to package the extension easily:
   ```bash
   npm run package
   ```
   
   This will create a `scrust-vscode-0.1.5.vsix` file in the directory.

5. **Install in VS Code**:
   - Open VS Code.
   - Go to the **Extensions** view (`Ctrl+Shift+X`).
   - Click the "..." (Views and More Actions) menu at the top right of the extension sidebar.
   - Select **Install from VSIX...**.
   - Navigate to and select the generated `.vsix` file.

### Development Mode

If you want to contribute to the extension or test changes without packaging:

1. Open the `editors/vscode` folder in VS Code.
2. Press `F5` to launch a new VS Code window with the extension loaded (Extension Development Host).
3. Open any `.sr` file to test the features.
