# blutils

What is "Blutils"? Blutils is a set of coretutils meant for everyday use.
The name Blutils is a wordplay between the color "blue" and the word "utils".

Blutils aims to improve the experience of your system with lightwheight, :sparkles:fast:sparkles: and good looking visuals.

Except just being amazing, Blutils also strives to be compatible with existing GNU coreutils. This means that Blutils brings QOL-features while feeling somewhat familiar!

NOTE: Blutils is compatible in almost every way, except SELinux and SMACK functionality.

This set of coretutils has a lot of work left to do, but check out the Implementation tracker down below to find out what utils are done!

# Building

For the moment there's no way to install or build this set of utils. There is a "build.py" script but it wont symlink anything, so really you're on your own if you want to install this.

When I feel that it is in an acceptable state I'll provide an installation method!

But, if you want to try it out anyways, read this!

## How to actually test out blutils

The most solid way I can recommend to try out blutils is by using the command "cargo run".

Here's the commands you'll need to try it out:

```
# help page
cargo run ( or ) cargo run -- --help
# cat
cargo run -- cat
# mkdir
cargo run -- mkdir
# rmdir
cargo run -- rmdir
# mv
cargo run -- mv
# cp
cargo run -- cp
# ln
cargo run -- ln
# rm
cargo run -- rm
# chown
cargo run -- chown
# chmod
cargo run -- chmod
```

\*NOTE: All of these commands do what they are supposed to do. This can lead to **_destructive_** actions.\*

**TREAT THESE COMMANDS AS YOU WOULD TREAT YOUR NORMAL COMMANDS!**

# Conributing

[Click here for the guide](/CONTRIBUTING.md)

# Tracker

[Click here for the tracker file](/TRACKER.md)
