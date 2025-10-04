#!/bin/python3

# The purpose of this script is to verify that all students,
# at the end of a lesson, committed a 'substantial' amount of
# changes to their code base.

# USAGE: ./git-diff-guard <PARENT DIRECTORY>

from datetime import date, timedelta
import git
import os
import sys

SUBSTANTIAL_COMMIT_SIZE = 50

def analyze_sub_dir(path, project_name):
    g = git.Git(path)
    loginfo: str = g.log(f'--since={today_str}', '--pretty=tformat:', '--numstat', '--all')

    if len(loginfo) == 0:
        print(f"❌ FAIL: {project_name}")
        return (0, 0, False)

    # until we find a better alternative:
    # we receive a list of lines that contain the diff in the following format:
    # LINES_ADDED    LINES_REMOVED    MODIFIED_FILE

    added = 0
    removed = 0

    log_lines = loginfo.split("\n")
    for line in log_lines:
        log = line.split(None)

        added += int(log[0])
        removed -= int(log[1])

    total_change_count = abs(added) + abs(removed)
    passes_minimum = total_change_count >= SUBSTANTIAL_COMMIT_SIZE

    if passes_minimum:
        print (f"✅ PASS: {project_name}")
    else:
        print(f"❌ FAIL: {project_name}")

    return (added, removed, passes_minimum)
    

if len(sys.argv) < 2:
    print("USAGE: ./git-diff-guard <PARENT DIRECTORY>")
    exit(1)

parent_dir = sys.argv[1]
sub_folders = [ f.path for f in os.scandir(parent_dir) if f.is_dir() ]

today_str = (date.today() - timedelta(days=1)).strftime("%Y-%m-%d")

with open("logs.csv", "a") as log_file:
    for sub_dir in sub_folders:
        project_name = os.path.basename(sub_dir)[13:].lower()
        stats = analyze_sub_dir(sub_dir, project_name)

        log_file.write(f'{today_str};{project_name};{stats[0]};{stats[1]};{ 'PASS' if stats[2] else 'FAIL' }\n')

