complete -c find-identical-files -s a -l algorithm -d 'Choose the hash algorithm' -r -f -a "{ahash	'',blake3	'',fxhash	'',sha256	'',sha512	''}"
complete -c find-identical-files -s b -l min_size -d 'Set a minimum file size (in bytes) to search for identical files' -r
complete -c find-identical-files -s B -l max_size -d 'Set a maximum file size (in bytes) to search for identical files' -r
complete -c find-identical-files -s c -l csv_dir -d 'Set the output directory for the CSV file (fif.csv)' -r -F
complete -c find-identical-files -s d -l min_depth -d 'Set the minimum depth to search for identical files' -r
complete -c find-identical-files -s D -l max_depth -d 'Set the maximum depth to search for identical files' -r
complete -c find-identical-files -s f -l min_frequency -d 'Minimum frequency (number of identical files) to be filtered' -r
complete -c find-identical-files -s F -l max_frequency -d 'Maximum frequency (number of identical files) to be filtered' -r
complete -c find-identical-files -s g -l generate -d 'If provided, outputs the completion file for given shell' -r -f -a "{bash	'',elvish	'',fish	'',powershell	'',zsh	''}"
complete -c find-identical-files -s i -l input_dir -d 'Set the input directory where to search for identical files [default: current directory]' -r -F
complete -c find-identical-files -s r -l result_format -d 'Print the result in the chosen format' -r -f -a "{json	'',yaml	'',personal	''}"
complete -c find-identical-files -s x -l xlsx_dir -d 'Set the output directory for the XLSX file (fif.xlsx)' -r -F
complete -c find-identical-files -s e -l extended_path -d 'Prints extended path of identical files, otherwise relative path'
complete -c find-identical-files -s o -l omit_hidden -d 'Omit hidden files (starts with \'.\'), otherwise search all files'
complete -c find-identical-files -s s -l sort -d 'Sort result by number of identical files, otherwise sort by file size'
complete -c find-identical-files -s t -l time -d 'Show total execution time'
complete -c find-identical-files -s v -l verbose -d 'Show intermediate runtime messages'
complete -c find-identical-files -s w -l wipe_terminal -d 'Wipe (Clear) the terminal screen before listing the identical files'
complete -c find-identical-files -s h -l help -d 'Print help (see more with \'--help\')'
complete -c find-identical-files -s V -l version -d 'Print version'
