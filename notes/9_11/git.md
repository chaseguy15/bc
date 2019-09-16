# Git

## Installation

To download git, simply type this in the unix shell:
`$ sudo apt install git-all`
Before the next part, make sure you have set up a github account with your UT email address.

## Status
`$ git status`
This is a useful tool for determining the differences between the local and remote repository without referencing the origin directly.

## Pull
```bash
$ git status
$
$
```

## Push
```bash
$ git add .
$ git status
$ git commit -m "commit message"
$ git push
```
This is the standard formula for pushing all local changes to the origin. "add" puts your files into the staging area. Here a period is used to represent all files. Alternatively, to add individual files or directories, you can type:
`$ git add myfile.rs`

"commit" can be thought of as a save feature, moving all of the staged changes into the local commit history. This updates the local repo to show the staged changes. Finally, "push" is used to merge the local repo into the origin. This effectively publishes your work, making your origin up to date with your latest local commit.

## Branching

To add a new branch to your local repo and to remote, use this:
```bash
$ git checkouut -b branch_name
$ git push -u origin branch_name
```

to switch to a new branch on the origin, use this:
```bash
$ git 
```
