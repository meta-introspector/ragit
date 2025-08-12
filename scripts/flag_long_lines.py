import sys
import os

def flag_long_lines(file_path, limit):
    try:
        with open(file_path, 'r') as f:
            for i, line in enumerate(f):
                if len(line.strip()) > limit:
                    # Shorten the path by removing the common root prefix
                    shortened_line = line.replace("/data/data/com.termux/files/home/storage/github/ragit/", "")
                    print(f"Line {i+1} (length {len(line.strip())}): {shortened_line.strip()}")
    except FileNotFoundError:
        print(f"Error: File not found at {file_path}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python flag_long_lines.py <file_path> <line_length_limit>")
        sys.exit(1)

    file_path = sys.argv[1]
    try:
        line_length_limit = int(sys.argv[2])
    except ValueError:
        print("Error: Line length limit must be an integer.")
        sys.exit(1)

    flag_long_lines(file_path, line_length_limit)