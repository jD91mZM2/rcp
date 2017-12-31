# RCP

A file copy utility, written in Rust

# Usage

Just replace normal `cp` commands with `rcp`.  
Many flags aren't implemented yet, sadly.

# Copy sink!

`rcp` supports copying and pasting in separate runs!  
Simply run `rcp <source>` without a destination, and later run `rcp -p` to paste all the things!
```
$ cd dir1
dir1 $ rcp file1
dir1 $ rcp file2
dir1 $ cd ../dir2
dir2 $ rcp -p
# dir2 now contains file1 and file2
```
