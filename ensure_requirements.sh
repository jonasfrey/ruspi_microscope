# all requirements should be listed here 

# curl -fsSL https://deno.land/install.sh | sh

apt install python3 

apt install pip3 

pip3 install imutils

#!/bin/sh

# Check if Deno is installed
if ! command -v deno &> /dev/null; then
    echo "Deno not found. Installing..."
    curl -fsSL https://deno.land/x/install/install.sh | sh

    # Add Deno to PATH in .bashrc
    DENO_INSTALL="\$HOME/.deno"
    DENO_PATH='export DENO_INSTALL="$HOME/.deno"\nexport PATH="\$DENO_INSTALL/bin:\$PATH"'

    if ! grep -q 'export DENO_INSTALL=' "$HOME/.bashrc"; then
        echo "Updating .bashrc to include Deno..."
        echo -e "\n# Added by Deno install script\n$DENO_PATH" >> "$HOME/.bashrc"
    fi

    # Source the .bashrc file to update the current shell session
    . "$HOME/.bashrc"
else
    echo "Deno is already installed."
fi

# Check if the 'requests' Python package is installed
if ! python3 -m pip show requests &> /dev/null; then
    echo "Python package 'requests' not found. Installing..."
    python3 -m pip install requests
else
    echo "Python package 'requests' is already installed."
fi

