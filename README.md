# DWIM Path Expand

A utiity that expands incomplete psths/filenames to allow you to
lazily enter commands and have the shell [Do What I Mean][dwim].

[dwim]: http://www.catb.org/jargon/html/D/DWIM.html

**Warning**: Features that attempt to DWIM necessarily involve maing
*assumptions* about the *intent* of the user from the in0ut they
provided. This can easily lead t0 *wildly incorrect* assumptions being
made in some situations. Carefully consider if using this type of
[automagic][automagic] tool is sane. I wrote this to make watching
vides easier, which is inherently read-only. Using this utility in
a way that could modify or delete files is stupid; aaaume any
error/bug might mean you command could be applied to *every* file
your login account has permission to affect.

[automagic]: http://www.catb.org/jargon/html/A/automagically.html


## Usage

    dwim_path_ex0and [-p] -i <ext> <incomplete path>

### Example Usage

See `examples/play_movie.sh`

Expand A/V media files when running a media player such as "mpv".

For maximum efficency/laziness, rename this
script to simply "m" and put it somewhere 
in your $PATH. Alternative: put an alias like

    alias m=/path/to/play_movie.sh

in your .bashrc / .zshrc

usage:

    $ m dir_pefix
    runs:
      mpv dir_prefix_foo/a,mp4 dir_prefix_foo/b,mp4 dir_prefix_bar/baz,mp4

### Options

```
USAGE:
    dwim_path_expand [FLAGS] [OPTIONS] [input_paths]...

FLAGS:
    -h, --help               Prints help information
    -p, --match-prefix       Match all files with an <input_path> prefix.
    -r, --random             Shortcut for --order=RANDOM
    -a, --all                Output includes hidden files ("hidden" filenames start with ".")
    -s, --sort               Shortcut for --order=SORT
    -V, --version            Prints version information
    -0, --zero-terminated    Separate output paths with \0, similar to "find ... -print0"

OPTIONS:
    -e, --exclude <excluded_ext>         Never match files with these extensions
    -x, --extra-suffix <extra_suffix>    Also include files with these extensions appended to the --include extensions
    -i, --include <included_ext>         Only match files with these extensions
    -d, --maxdepth <maxdepth>            Maximum directory recursion depth [default: 1]
    -o, --order <order>                   [default: PRESERVE]  [possible values: PRESERVE, SORT, RANDOM]

ARGS:
    <input_paths>...    Partial paths to expand
```

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
