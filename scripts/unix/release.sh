#!/bin/bash


usage() {
    echo "Usage: $0 -k SSH_KEY_FILE -h REMOTE_HOST -u REMOTE_USER -p REMOTE_PATH [--install]"
    echo "  -k: Path to SSH key file"
    echo "  -h: Remote hostname"
    echo "  -u: Remote username"
    echo "  -p: Remote deploy path"
    echo "  --install: Install web dependencies (optional)"
    exit 1
}

for arg in "$@"; do
    case "$arg" in
        --install)
            INSTALL_DEPS=true
            shift # Remove --install from arguments
            ;;
    esac
done

while getopts "k:h:u:p:" opt; do
    case $opt in
        k) SSH_KEY_FILE="$OPTARG";;
        h) REMOTE_HOST="$OPTARG";;
        u) REMOTE_USER="$OPTARG";;
        p) REMOTE_PATH="$OPTARG";;
        ?) usage;;
    esac
done

if [ -z "$SSH_KEY_FILE" ] || [ -z "$REMOTE_HOST" ] || [ -z "$REMOTE_USER" ] || [ -z "$REMOTE_PATH" ]; then
    echo "Error: Missing required arguments"
    usage
fi

ssh-add -k "$SSH_KEY_FILE"

if [ "$INSTALL_DEPS" = true ]; then
    SCRIPT_PATH="./scripts/unix/web-dependencies.sh"

    if [ ! -f "$SCRIPT_PATH" ]; then
        echo "Error: Script not found at $SCRIPT_PATH"
        exit 1
    fi

    if [ ! -x "$SCRIPT_PATH" ]; then
        echo "Error: Script is not executable. Adding execute permission..."
        chmod +x "$SCRIPT_PATH"
    fi

    "$SCRIPT_PATH"

    if [ $? -eq 0 ]; then
        echo "web dependencies installed successfully"
    else
        echo "failed to install web dependencies with exit code $?"
        exit 1
    fi
fi

cargo build --release --target x86_64-unknown-linux-musl

if [ $? -eq 0 ]; then
    echo ""
    echo "Cargo build completed successfully"
    echo ""
else
    echo "Cargo build failed with exit code $?"
    exit 1
fi

check_and_upload() {
    local file=$1
    if [ ! -f "$file" ]; then
        echo "Error: File not found: $file"
        return 1
    fi

    echo "Uploading $file..."
    scp -o StrictHostKeyChecking=no -o ConnectTimeout=10 -i "$SSH_KEY_FILE" "$file" "$REMOTE_USER@$REMOTE_HOST:release"

    if [ $? -eq 0 ]; then
        echo "Successfully uploaded $file"
    else
        echo "Failed to upload $file"
        return 1
    fi
}

echo "Creating remote directory..."
ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 -i "$SSH_KEY_FILE" "$REMOTE_USER@$REMOTE_HOST" "mkdir -p release"

if [ $? -ne 0 ]; then
    echo "Failed to create remote directory"
    exit 1
fi

echo "Starting file uploads..."

FILES_TO_UPLOAD=(
    "web/regexes.yaml"
    "web/web_files.zip"
    "api/api_victorhqc_com.db"
    "target/x86_64-unknown-linux-musl/release/api-victorhqc-com"
    "target/x86_64-unknown-linux-musl/release/web-victorhqc-com"
)

for file in "${FILES_TO_UPLOAD[@]}"; do
    check_and_upload "$file"
    if [ $? -ne 0 ]; then
        echo "Upload process failed"
        exit 1
    fi
done

echo ""
echo "All files uploaded successfully!"
echo ""


echo "Preparing service and restarting..."
ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 -i "$SSH_KEY_FILE" "$REMOTE_USER@$REMOTE_HOST" <<'EOF'

    echo "Moving binaries"
    mv release/api-victorhqc-com linux-api-victorhqc-com
    mv release/web-victorhqc-com linux-web-victorhqc-com

    echo "Moving DB"
    cp api_victorhqc_com.db api_victorhqc_com_bkp_$(date +%Y%m%d_%H%M%S).db
    rm api_victorhqc_com.db*
    mv release/api_victorhqc_com.db api_victorhqc_com.db
    chmod 775 api_victorhqc_com.db

    echo "Cleaning static files"
    rm -rf victorhqc.com/*

    unzip -o release/web_files.zip -d victorhqc.com

    echo "Moving regexes.yaml"
    mv release/regexes.yaml victorhqc.com/regexes.yaml

    echo "Restarting services"
    sudo systemctl restart www.victorhqc.com
    sudo systemctl restart api2.victorhqc.com
EOF
