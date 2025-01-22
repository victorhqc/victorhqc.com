#!/bin/sh


TAILWIND_URL="https://cdn.tailwindcss.com"
HTMX_URL="https://unpkg.com/htmx.org@2.0.3/dist/htmx.min.js"

OUTPUT_FOLDER="web/static"
TAILWIND_FILE="$OUTPUT_FOLDER/tailwindcss.js"
HTMX_FILE="$OUTPUT_FOLDER/htmx.js"

echo "Ensuring folder $OUTPUT_FOLDER exists..."
mkdir -p "$OUTPUT_FOLDER"

echo "Downloading contents from $TAILWIND_URL..."
wget -O "$TAILWIND_FILE" "$TAILWIND_URL"

if [[ $? -eq 0 ]]; then
    echo "Downloaded successfully. Saved as $TAILWIND_FILE."
else
    echo "Failed to download from $TAILWIND_URL."
    exit 1
fi

echo "Downloading contents from $HTMX_URL..."
wget -O "$HTMX_FILE" "$HTMX_URL"

if [[ $? -eq 0 ]]; then
    echo "Downloaded successfully. Saved as $HTMX_FILE."
else
    echo "Failed to download from $HTMX_URL."
    exit 1
fi

LIBS="$OUTPUT_FOLDER/victorhqc.com.libs"
git clone https://github.com/victorhqc/victorhqc.com.libs.git $LIBS

cd $LIBS
git pull

deno run --allow-env --allow-read --allow-write --allow-run ./bundle.ts
npx terser dist/photo-stack.js -o dist/photo-stack.min.js -c -m
npx tailwindcss -i ./src/photo-stack/styles.css -o ./dist/photo-stack.min.css --minify

cd - > /dev/null

mv "$LIBS/dist/photo-stack.min.js" "$OUTPUT_FOLDER/photo-stack.min.js"
mv "$LIBS/dist/photo-stack.min.css" "$OUTPUT_FOLDER/photo-stack.min.css"
rm -rf "$OUTPUT_FOLDER/victorhqc.com.libs"
