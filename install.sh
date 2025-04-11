#!/bin/bash

# Variables
REPO_URL="https://github.com/annyman/weavr.git" # Replace with your repo URL
APP_NAME="weavr"                                      # Replace with your app name
DESKTOP_FILE="/usr/share/applications/${APP_NAME}.desktop"
INSTALL_DIR="/usr/bin"

# Function to print messages
function print_message() {
    echo -e "\033[1;32m$1\033[0m"
}

# Step 1: Clone the repository
print_message "Cloning the repository..."
git clone "$REPO_URL" || { echo "Failed to clone repository."; exit 1; }
cd "$APP_NAME" || { echo "Failed to enter project directory."; exit 1; }

# Step 2: Install Rust if not already installed
if ! command -v cargo &> /dev/null; then
    print_message "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh || { echo "Failed to install Rust."; exit 1; }
    source "$HOME/.cargo/env"
fi

# Step 3: Build the binary using Cargo
print_message "Building the project using Cargo..."
cargo build --release || { echo "Failed to build project."; exit 1; }

# Step 4: Move the binary to /usr/bin/
print_message "Installing the binary to ${INSTALL_DIR}..."
sudo mv target/release/"$APP_NAME" "$INSTALL_DIR" || { echo "Failed to move binary."; exit 1; }
sudo chmod +x "${INSTALL_DIR}/${APP_NAME}" || { echo "Failed to set executable permissions."; exit 1; }

# Step 5: Create a .desktop file (optional for CLI integration)
print_message "Creating .desktop file for integration..."
sudo bash -c "cat > $DESKTOP_FILE" <<EOL
[Desktop Entry]
Name=${APP_NAME}
Exec=${INSTALL_DIR}/${APP_NAME}
Terminal=true
Type=Application
Icon=utilities-terminal
Categories=Development;
EOL

print_message "Setting permissions for .desktop file..."
sudo chmod +x "$DESKTOP_FILE"

# Step 6: Clean up
print_message "Cleaning up..."
cd ..
rm -rf "$APP_NAME"

# Final Message
print_message "${APP_NAME} has been successfully installed!"
