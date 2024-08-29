# Contributing

Hey! You want to conribute? Well you're in the right place!

Just follow the guide below and you'll be a contributor in no time!

# THE Guide!

## Guidelines

When working on or creating a command we follow some guidelines. 

These guidelines are in place to make sure that the next person reading your code doesn't get a stroke while trying to read it!

- Keep comments and PR:s civil and nice. We don't want to hurt anyone's feelings, no matter how bad the commit or code!
- Prefer readbility over functionality. Once again, we don't want anyone getting a stroke reading it
- Always explain what you have done and why in your commits. Examples of bad commit messages:
    - "Fixed bug" (Doesn't describe what has been done)
    - "." (Empty message is bad)
    - "John Doe caused this and now I blame him for it. I fixed it" (Finger pointing which breaks guideline 1)
- Always prioritize GNU Coreutils compability. If this is not possible please give a reason why this is the case.
- Commits in rust are preferred but any compiled language is allowed as long as it plays nicely with the rest of the project
    - If it changes the build procedure in any way you need state this in the commit, **CLEARLY**
    - And please, no Python or Javascript for commands.
- Lastly but not least, never commit untested code. This causes a headache for other contributors while only causing harm to the project

Now, with the boring stuff out of the way, let's get started with contributions!

## Working on an existing command

As stated above you can work in almost any language, but the root will almost always be in rust except explicitly stated otherwise. 

In every command there's a command/clap arguments declaration. For every argument there will be a comment stating "TODO" or "Done". 
In rare cases there will be more to the comment. 

**IF THERE ARE NO COMMENTS THE COMMAND IS MOST LIKELY DONE AND YOU SHOULD FOCUS ON IMPROVING IT**

## Creating a new command

Follow this guide when creating a new command:
1. Make a copy of the template file (src/template.rs) with the command name.
2. Import the file into the main.rs file with mod. Next add it to the match statement. Look at other commands for reference!
3. Start adding all options and make sure the types are right

## Utils

If you find that you need something specific not in the std lib and that you can implement it yourself please consider adding it to the utils lib.

This lib contains functions, types and traits used by the project in various ways, but it also implements cool stuff you can use outside of this project.
It only depends on the standard lib, the libc crate and the nix crate.

So, please consider helping out improving the utilities lib, and if you find bugs you are **VERY** appreciated if you fix them!

# Ending notes

Well, that's it. If you have any more questions mail me at `spamixproducer@gmail.com` or open a discussion in github discussions.

Thanks for reading this far, and happy hacking!

// Alexander aka "Sapmix"/"SpamixOfficial"

# Improving the contributions guide

Side note here. If you find that something is missing or that this document can be improved, please do!
