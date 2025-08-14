# Retitle

A cli tool to bulk rename files. 

```
Retitle, simple tool to bulk rename files in the current directory. By default it'll open a text editor to edit the files to their new names. Alternate options are to read and write the list of operations from stdin/out or file A retitle rename line is <original_name>|<new_name>. One per line

Usage: retitle [OPTIONS]

Options:
  -o, --stdout               Output to stdout
  -i, --stdin                Input from stdin
  -e, --file-out <FILE_OUT>  Exports to file
  -r, --file-in <FILE_IN>    Resume from file
  -h, --help                 Print help
```