#!/usr/bin/env python
import os

# get current directory
directory = os.getcwd()

# loop through all directories and files in the directory
for root, dirs, files in os.walk(directory):
    for filename in files:
        # get full path of file
        file_path = os.path.join(root, filename)
        # check if it's a file
        if not filename.endswith(".esp") and not filename.endswith(".esm") and not filename.endswith(".bsa"):
            # get new lowercase name
            new_name = filename.lower()
            # rename file
            os.rename(file_path, os.path.join(root, new_name))
