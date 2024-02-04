#!/bin/bash

# Expands A/V media files. Use this with a media
# player (such as "mpv").
#
# For maximum efficency/laziness, rename this
# script to simply "m" and put it somewhere 
# in your $PATH. Alternative: put an alias like
#   alias m=/path/to/play_movie.sh
# in your .bashrc / .zshrc
#
# usage:
#    $ m dir_pefix
#    runs:
#      mpv dir_prefix_foo/a,mp4 dir_prefix_foo/b,mp4 dir_prefix_bar/baz,mp4

PLAYER="mpv"

VIDEO_TYPES="avi,flv,m4v,mkv,mov,mp4,mpg,ogm,webm,wmv"
AUDIO_TYPES="flac,m4a,mp3,ogg,wav"
TYPES="${VIDEO_TYPES},${AUDIO_TYPES}"

EXTRA_SUFFIX_TYPES="part"

expand_media_files() {
    dwim_path_expand \
        --zero-terminated \
        --match-prefix \
        --include "${TYPES}" \
        --extra-suffix "${EXTRA_SUFFIX_TYPES}" \
        "$@"
}

declare -a media_files=()

while IFS=$'\0' read -r file
do
    media_files+=( "${file}" )
done < <(expand_media_files "$@")

exec "${PLAYER}" "${media_files[@]}"

