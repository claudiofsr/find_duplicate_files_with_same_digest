#compdef find-identical-files

autoload -U is-at-least

_find-identical-files() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-a+[Choose the hash algorithm]:ALGORITHM:(ahash blake3 fxhash sha256 sha512)' \
'--algorithm=[Choose the hash algorithm]:ALGORITHM:(ahash blake3 fxhash sha256 sha512)' \
'-b+[Set a minimum file size (in bytes) to search for identical files]:MIN_SIZE: ' \
'--min_size=[Set a minimum file size (in bytes) to search for identical files]:MIN_SIZE: ' \
'-B+[Set a maximum file size (in bytes) to search for identical files]:MAX_SIZE: ' \
'--max_size=[Set a maximum file size (in bytes) to search for identical files]:MAX_SIZE: ' \
'-c+[Set the output directory for the CSV file (fif.csv)]:CSV_DIR:_files' \
'--csv_dir=[Set the output directory for the CSV file (fif.csv)]:CSV_DIR:_files' \
'-d+[Set the minimum depth to search for identical files]:MIN_DEPTH: ' \
'--min_depth=[Set the minimum depth to search for identical files]:MIN_DEPTH: ' \
'-D+[Set the maximum depth to search for identical files]:MAX_DEPTH: ' \
'--max_depth=[Set the maximum depth to search for identical files]:MAX_DEPTH: ' \
'-f+[Minimum frequency (number of identical files) to be filtered]:MIN_FREQUENCY: ' \
'--min_frequency=[Minimum frequency (number of identical files) to be filtered]:MIN_FREQUENCY: ' \
'-F+[Maximum frequency (number of identical files) to be filtered]:MAX_FREQUENCY: ' \
'--max_frequency=[Maximum frequency (number of identical files) to be filtered]:MAX_FREQUENCY: ' \
'-g+[If provided, outputs the completion file for given shell]:GENERATOR:(bash elvish fish powershell zsh)' \
'--generate=[If provided, outputs the completion file for given shell]:GENERATOR:(bash elvish fish powershell zsh)' \
'-i+[Set the input directory where to search for identical files \[default\: current directory\]]:INPUT_DIR:_files' \
'--input_dir=[Set the input directory where to search for identical files \[default\: current directory\]]:INPUT_DIR:_files' \
'-r+[Print the result in the chosen format]:RESULT_FORMAT:(json yaml personal)' \
'--result_format=[Print the result in the chosen format]:RESULT_FORMAT:(json yaml personal)' \
'-x+[Set the output directory for the XLSX file (fif.xlsx)]:XLSX_DIR:_files' \
'--xlsx_dir=[Set the output directory for the XLSX file (fif.xlsx)]:XLSX_DIR:_files' \
'-e[Prints extended path of identical files, otherwise relative path]' \
'--extended_path[Prints extended path of identical files, otherwise relative path]' \
'-o[Omit hidden files (starts with '\''.'\''), otherwise search all files]' \
'--omit_hidden[Omit hidden files (starts with '\''.'\''), otherwise search all files]' \
'-s[Sort result by number of identical files, otherwise sort by file size]' \
'--sort[Sort result by number of identical files, otherwise sort by file size]' \
'-t[Show total execution time]' \
'--time[Show total execution time]' \
'-v[Show intermediate runtime messages]' \
'--verbose[Show intermediate runtime messages]' \
'-w[Wipe (Clear) the terminal screen before listing the identical files]' \
'--wipe_terminal[Wipe (Clear) the terminal screen before listing the identical files]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
}

(( $+functions[_find-identical-files_commands] )) ||
_find-identical-files_commands() {
    local commands; commands=()
    _describe -t commands 'find-identical-files commands' commands "$@"
}

if [ "$funcstack[1]" = "_find-identical-files" ]; then
    _find-identical-files "$@"
else
    compdef _find-identical-files find-identical-files
fi
