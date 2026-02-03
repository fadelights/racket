#!/usr/bin/env bash

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
