# Tracker

## Current planned utils

| Name  | Description                                       | Status             |
| ----- | ------------------------------------------------- | ------------------ |
| ls    | list directory                                    | :construction:     |
| cat   | concatenate files to stdout                       | :white_check_mark: |
| mkdir | make directory                                    | :white_check_mark: |
| rmdir | remove empty directory                            | :white_check_mark: |
| rm    | remove files or directories                       | :white_check_mark: |
| cp    | copy files or directories                         | :white_check_mark: |
| mv    | move files or directories                         | :white_check_mark: |
| ln    | make hard or symbolic links                       | :white_check_mark: |
| chown | change file owner and group                       | :white_check_mark: |
| chmod | change file permissions                           | :white_check_mark: |
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
| rm    | 90%     | preserve-root                             |
| ln    | 93%     | relative                                  |
| chown | 100%    | None!                                     |
| chmod | 100%    | None!                                     |
