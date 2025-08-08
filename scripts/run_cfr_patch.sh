#!/bin/bash
# SOP 2: Running a Semantic Patch with `cfr`

# Purpose: To apply a semantic patch to a target file or directory using the `cfr` tool.

# Procedure:
# 1. Ensure `cfr` is installed and accessible in your PATH (refer to SOP 1).
# 2. Prepare your semantic patch file (.cocci) and identify your target Rust file/directory.
# 3. Execute the `cfr` command with the appropriate arguments.

# Usage: ./run_cfr_patch.sh <COCCI_FILE> <TARGET_PATH> [OUTPUT_FILE]
#   <COCCI_FILE>: Path to your semantic patch file (e.g., my_patch.cocci)
#   <TARGET_PATH>: Path to the Rust file or directory to apply the patch to (e.g., src/main.rs or src/)
#   [OUTPUT_FILE]: Optional. Path to save the transformed file. If not provided, changes are printed to stdout.

COCCI_FILE="$1"
TARGET_PATH="$2"
OUTPUT_FILE="$3"

if [ -z "$COCCI_FILE" ] || [ -z "$TARGET_PATH" ]; then
  echo "Usage: $0 <COCCI_FILE> <TARGET_PATH> [OUTPUT_FILE]"
  exit 1
fi

COMMAND="cfr -c \"$COCCI_FILE\" \"$TARGET_PATH\""

if [ -n "$OUTPUT_FILE" ]; then
  COMMAND="$COMMAND -o \"$OUTPUT_FILE\""
  echo "Applying semantic patch '$COCCI_FILE' to '$TARGET_PATH' and saving output to '$OUTPUT_FILE'..."
else
  echo "Applying semantic patch '$COCCI_FILE' to '$TARGET_PATH' and printing output to stdout..."
fi

# Execute the command
eval $COMMAND

if [ $? -eq 0 ]; then
  echo "Semantic patch applied successfully."
else
  echo "Failed to apply semantic patch."
fi
