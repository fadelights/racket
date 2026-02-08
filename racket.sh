#!/usr/bin/env bash

# Processes multiple MP3 files in parallel using Racket.
#
# USAGE:
# ./racket.sh <directory>
#
# ARGUMENTS:
#   $1 - Directory path containing MP3 files to process, search is recursive
#
# DEPENDENCIES:
# - ffmpeg: For audio format conversion (MP3 <-> WAV)
# - racket: Audio processing tool
# - GNU parallel: For parallel processing of multiple files
#
# WORKFLOW:
# 1. Finds all .mp3 files in the specified directory
# 2. For each MP3 file:
#    - Converts MP3 to temporary WAV file
#    - Processes WAV file using racket
#    - Converts processed WAV back to MP3 with high quality (qscale:a 2)
#    - Cleans up temporary WAV files
# 3. Processes files in parallel using all available CPU cores (-j+0)
#
# OUTPUT:
# Creates *_processed.mp3 files in the same directory as the input files
#
# NOTES:
# - Original files are preserved
# - Temporary files are automatically cleaned up
# - FFmpeg errors are suppressed for cleaner output
# - Parallel processing uses all available CPU cores for maximum performance


process_file() {
    input_mp3="$1"
    base_name="${input_mp3%.mp3}"
    temp_wav="${base_name}_temp.wav"
    output_wav="${base_name}_processed.wav"
    output_mp3="${base_name}_processed.mp3"

    echo "Processing: $input_mp3"

    ffmpeg -i "$input_mp3" "$temp_wav" -y -loglevel error
    racket "$temp_wav" "$output_wav"
    ffmpeg -i "$output_wav" -codec:a libmp3lame -qscale:a 2 "$output_mp3" -y -loglevel error
    rm "$temp_wav" "$output_wav"

    echo "Completed: $output_mp3"
}

# Export the function so GNU parallel can use it
export -f process_file

find "${1}" -name "*.mp3" -type f | parallel -j+0 process_file {}
