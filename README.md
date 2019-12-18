# profilers (working title)
Goal of the project:  
An improved version of my old [Ruby script][profilerb]

## Description
Include or exclude (via comments) certain lines in config files,  
depending on the currently active _profiles_.  
A _profile_ is simply a string.  
The default _profile_ for a machine should be its `hostname`.  

The app needs to know for which _profile_ to run,  
and which _files_ to edit.  

## Usage
### In configs
In configs, write something like ...
```
# PROFILE = laptop
set only-on-laptop

# PROFILE = !desktop
set not-on-desktop

# PROFILE_START = desktop && !laptop
set on-desktop
set not-on-laptop
# PROFILE_END
```

To un-comment the line on a `laptop` profile, and comment-out the line on any other profile.

### Command-Line Usage
```
# Run with default profile and files (maybe disallow default files?)
$ profilers

# Run on the given files, with the default profile (current hostname)
$ profilers .bashrc .vimrc .config/i3/config

# Accept glob patterns
$ profilers .*rc

# Run on the files in the given directories (non-recursively)
$ profilers .config/i3 .config/i3/i3blocks

# Run on the files in the given directories (recursively)
$ profilers -r .config/i3
$ profilers --recurse .config/i3

# Run on the given files, with the 'main' profile
$ profilers -p main .bashrc .vimrc
$ profilers --profile main .bashrc .vimrc

# Run on the given files, with the 'main' and 'onedisp' profiles
$ profilers -p main -p onedisp .bashrc .vimrc
$ profilers --profile main .bashrc .vimrc

# Don't modify any files
$ profilers -n **/.*rc
$ profilers --no-modify **/.*rc

# Print info about what's happening to stderr
$ profilers -v .bashrc .vimrc
$ profilers --verbose .bashrc .vimrc
  Running on profile 'profile-name'
  Modifiying './.bashrc'
  Modifiying './.vimrc'

# Combine -n and -v
$ profilers -nv .bashrc .vimrc
  Running on profile 'profile-name'
  Modifiying './.bashrc'
  Modifiying './.vimrc'
  Performing no modifications on filesystem...
```

### Configuration
Config file under `~/.config/profilers/config.toml`

```toml
[keywords]
# For single-line profile encapsulation
single = "/^\s*#\s*PROFILE\s*=\s*(?P<expr>)$/"
# The named capture group `expr` will be parsed as the expression,
# which evaluates to a boolean.
# Expression would be something like ...
#   desktop
#   !desktop && laptop
# Example in a config file ...
#   # PROFILE = desktop
#   set only-on-desktop

# For multi-line profile encapsulation
block_start = "/^\s*#\s*PROFILE_START\s*=\s*(?P<expr>)$/"
block_end = "/^\s*#\s*PROFILE_END\s*$/"
# Example in a config file ...
#   # PROFILE_START = !laptop && desktop
#   set not-on-laptop
#   set on-desktop-not-laptop
#   # PROFILE_END

[files]
# Aliases for filepaths. Can be used on the command-line or in the `default` key
# below, by prefixing strings with `@`
aliases = {
    vim = ["~/.vimrc", "~/.config/nvim/init.vim"],
    i3 = "~/.config/i3",
}

# Default config files to run on, if no files were given as arguments
default = [
    "~/.bashrc",
    "@vim",
    "~/.config/i3/config",
]

[profiles]
# By default, the machine's hostname is used as the profile.
# If the hostname appears as a key in this table (`profiles.hostnames`),
# then the associated value is used as the profile / profiles.
hostnames = {
    aware-desktop = "aware-desktop",  # This should be redundant, as the hostname is the default profile
    laptop = ["laptop", "onedisp"],
}
```

[profilerb]: https://github.com/Noah2610/profile.rb
