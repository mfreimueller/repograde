# repograde

CLI tool for fetching and auto-grading GitHub classroom projects,
where "auto-grading" refers to a pass/fail scale, based on a minimum
commit size.

## Usage

Launch repograde in the directory that contains your students' repositories.
repograde will first perform `git fetch` on all student repositories and then,
using `git log` calculate the number of lines added/removed. The total number
of line changes is then compared to a configurable minimum commit size, which
decides whether a project passes or not.

The result is printed to stdout, as well as written into a CSV file whose 
location can be configured via the config file.

### Arguments

- **date** ... the cutoff date for calculating the diff in commits.
Defaults to yesterday, therefor calculating all lines added/removed since
yesterday. The format is *YYYY-MM-DD*.
- **config_file** ... the path to the config file. Defaults to `repograde.toml`

### Config File

Currently, some minor details are configurable via a toml file:

- **prefix** (String) ... as each student repository may have a common
prefix (for example: "team-project-"), you want the individual project name
to be a concise as possible, therefor the prefix is stripped away from the
directory names.
- **minimum_commit_size** (i32) ... the minimum amount of lines that have to be 
added/removed in total. Basically an arbitrary amount that should help you
figure out which students actually worked on their project and which didn't.
- **csv_file** (String) ... the location where the result should be written to. 