# Tracker

## Current planned utils

| Name  | Description                                     | Status            |
| ----- | ------------------------------------------------- | ------------------ |
| ls    | list directory                                    | :red_circle:       |
| cat   | concatenate files to stdout                       | :white_check_mark: |
| mkdir | make directory                                    | :white_check_mark: |
| rmdir | remove empty directory                            | :white_check_mark: |
| rm    | remove files or directories                       | :red_circle:       |
| cp    | copy files or directories                         | :white_check_mark: |
| mv    | move files or directories                         | :white_check_mark: |
| ln    | make hard or symbolic links                       | :red_circle:       |
| chown | change file owner and group                       | :red_circle:       |
| chmod | change file permissions                           | :red_circle:       |
| dd    | convert and copy a file                           | :red_circle:       |
| df    | report file system disk space usage               | :red_circle:       |
| du    | estimate disk space used by files and directories | :red_circle:       |

Please suggest more utilities you'd like to be included!

## Compability chart

| Name  | Percent | Options Missing                           |
| ----- | ------- | ----------------------------------------- |
| cat   | 100%    | None!                                     |
| mkdir | 60%     | -Z and --context                          |
| rmdir | 100%    | None!                                     |
| mv    | 93%     | Update options                            |
| cp    | 73%     | SELINUX and SMACK missing, Update options |
