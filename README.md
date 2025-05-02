# Build Instructions

This crate requires installation of necessary dependencies in order to build.
The crate is built-on top of PYo3 and requires at least python 3.9 and easyocr installed
either in the conda/pyenv/venv envirnoment or global level. If the build fails or receive "ModuleNotFound" error
at runtime try pointing the python to path `PYO3_PYTHON=/path/to/python cargo run`
