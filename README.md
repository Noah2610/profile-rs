# profile-rs (working title)
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

### Command-Line Usage
<details>
<summary>
Possible command-line usage mockup ...
</summary>

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
$ profilers --profile main --profile onedisp .bashrc .vimrc

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
</details>

### Configuration
Config file should be located under `~/.config/profilers/config.toml`.  
See the [default `config.toml` file][config].

[profilerb]: https://github.com/Noah2610/profile.rb
[config]:    ./config.toml
