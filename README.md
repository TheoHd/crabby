# crab-cleaner

CrabCleaner is a fast and safe way to manipulate files directly from your command line.

## Commands

```
.
└── John
    ├── Downloads/
    │   ├── file.txt
    │   ├── video.mp4
    │   └── photo.png
    ├── Videos/
    ├── Photos/
    └── common.crc
```

For all the commands example, we place ourselves in the `John` folder.

### Clean

```shell
crabcleaner -c . # Clean the current folder with the .crc file that is in the current folder
crabcleaner -c ./Downloads -f ./common.crc # Clean the Download folder with the common.crc file
```

## What is a .crc file ?

A `.crc` file is a configuration file for CrabCleaner. Its goal is to be executed on one or multiple folders, to clean them following some rules.

### Examples

Here, you can see an example of content for a `.crc` file :

```
// ===============
// Move commands
// ===============

// Bad
mv *.mp3                                              // This rule returns an error because "to" token is missing.
file.mp3 to C:/Users/Username/Documents/Music         // This rule returns an error because first token is missing.

// Good
mv file.mp3 to C:\Users\Username\Documents\Music      // This rule moves `file.mp3` to the Music folder

// ===============
// Prefix commands
// ===============

// Bad
pre *presentation.pdf                                 // This rule returns an error because "with" token is missing.
*presentation.pdf with urgent                         // This rule returns an error because first token is missing.

// Good
pre *presentation.pdf with urgent                     // This rule adds `urgent` to the beginning of all files that finishes with `presentation.pdf`.

// ===============
// Suffix commands
// ===============

// Bad
suf *presentation.pdf                                 // This rule returns an error because `with` token is missing
*presentation with urgent                             // This rule returns an error because first token is missing

// Good
suf *presentation.pdf with _urgent                    // This rule adds `_urgent` at the end of all files that finishes with `presentation.pdf`.
```
### Ideas

- Plug to `watchman`to have a `--daemon` mode
- Create a `crabcleaner.service`

### TODO

#### July 2020 v0.1.0

- [X] "mv _ to _" pair.
- [X] "pre _ with _" pair.
- [X] "suf _ with _" pair.
- [X] Add explanation for .crc file
- [X] Add a dry-run mode
- [X] Add interactive mode

#### August 2020 v0.2.0

- [X] Replace image in README.md by ASCII Art
- [ ] Add terminal UI
- [ ] Add rn _ by command
```
// ===============
// Rename commands
// ===============

// Bad
rn presentation*.pdf                                  // This rule returns an error because "by" token is missing
presentation*pdf by Presentation*.pdf                 // This rule returns an error because first token is missing

// Good
rn presentation*.pdf by Presentation*.pdf             // This rule renames all files that starts with `presentation` by `Presentation`
```
- [ ] Add unzip | unzip _ to command
```
// Unzip commands
unzip *.zip                                           // This rule returns an error because `to` token is missing.
*.zip to %USERPROFILE%/Documents/Music                // This rule returns an error because first token is missing.

unzip *.zip to .                                      // This rule unzip all `.zip` files in their respective folders
unzip *music.zip to %USERPROFILE%/Documents/Music     // This rule unzip all files that finishes by `music.zip` in the Music folder.
```
- [ ] Write a parser of the .crc syntax using https://github.com/Geal/nom instead of the current pattern matching.
- [ ] Add del command.
- [ ] Add global variables on Windows (e.g. %USERPROFILE%).
- [ ] Add "/" on Windows for absolute paths.

#### September 2020 v0.3.0

- [ ] Add `crabcleaner -c ./* # Clean the current folder and all the subfolders with one level of depth`
- [ ] Add `crabcleaner -c ./*/* # Clean the current folder and all the subfolders with two level of depth`
- [ ] Add `crabcleaner -c ./** # Clean the current folder and all the subfolders with max level of depth`
- [ ] Add Linux support.
- [ ] Add Mac support.

#### October 2020 v0.4.0

- [ ] Add feature to loop directly on some folders to automatically apply rules

## Contributors

- [ElifCilingir](https://github.com/ElifCilingir)
- [hakimMzabi](https://github.com/hakimMzabi)
- [ch-ang](https://github.com/ch-ang)
- [yvan-sraka](https://github.com/yvan-sraka)