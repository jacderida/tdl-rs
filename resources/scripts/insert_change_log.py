#!/usr/bin/env python

import sys

def get_change_log_for_version(version):
    change_log = ""
    finished_reading_change_log = False
    with open("../../CHANGELOG.md", mode="r") as change_log_file:
        while not finished_reading_change_log:
            line = change_log_file.readline()
            if not line:
                break
            if version in line:
                while True:
                    change_log_line = change_log_file.readline()
                    if not change_log_line:
                        finished_reading_change_log = True
                        break
                    if change_log_line.startswith("## Version"):
                        finished_reading_change_log = True
                        break
                    change_log += change_log_line
    return change_log

def insert_change_log(change_log):
    release_description = ""
    with open("../../release_description.txt", mode="r+") as release_desc_file:
        release_description = release_desc_file.read().replace("__CHANGELOG__", change_log)
        release_desc_file.seek(0)
        release_desc_file.write(release_description)
        release_desc_file.truncate()

if __name__ == '__main__':
    version = sys.argv[1]
    change_log = get_change_log_for_version(version)
    insert_change_log(change_log)
