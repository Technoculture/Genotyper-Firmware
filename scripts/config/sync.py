#!/usr/bin/python3
# Make a request to a cloud file storage service
# - Get the latest version.txt file
# - Compare the version number to the local version
# - If the version number is newer, download the new version

import os, sys
import shutil

def copy_directory(src: str, dst: str) -> None:
    '''
    Copy a directory from src directory to dst directory
    '''
    if not os.path.exists(src):
        print(f"Source directory {src} does not exist.")
        return
    try:
        shutil.copytree(src, dst, dirs_exist_ok=True)
        print(f"Successfully copied {src} to {dst}.")
    except Exception as e:
        print(f"An error occurred while copying: {e}")

if __name__ == '__main__':
    # Copy genotyper folder to $HOME/genotyper
    src_dir = "genotyper"
    dst_dir = os.path.join(os.path.expanduser("~"), "genotyper")
    copy_directory(src_dir, dst_dir)

