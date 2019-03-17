% HTMLGREP(1)
% Andreas Tolfsen <ato@sny.no>
% March 2019

# NAME

htmlgrep - search HTML documents by CSS selectors

# SYNOPSIS

**htmlgrep** [**-c0h**] [**-m** _num_] [**--help**] [**--version**] \<pattern\> [\<file\>...]

# DESCRIPTION

**snowplough** is a simple-minded tool which creates indices and
body-searches vast quantities of ServiceNow tickets, which is useful to
find out how someone before you solved one particular problem.

# OPTIONS

**-c**, **--count**
: Only a count of selected lines is written to standard output.

**-0**
:

**-h**
: Never print filename headers (i.e. filenames) with output lines.

**--help**
: Display summary of options.

**-m _num_**, **--max-count**=_num_
: Stop after _num_ matches.

**--version**
: Display version information.

# EXAMPLES

# SEE ALSO

grep(1), sed(1), awk(1)
