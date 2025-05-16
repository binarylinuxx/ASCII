# ASCII
![ascii](ASCII.png)
EASY RUSTED ASCII GEN TOOL FROM PNG IMAGES

# INSTALL
INSTALL WELL PRETTY SIMPLE MAKE SURE CARGO INSTALLED
```
$ git clone https://github.com/binarylinuxx/ASCII
$ cd ~/ASCII
$ make build
$ sudo make install
```

# USAGE
```
Convert images to ASCII art

Usage: ASCII [OPTIONS] --xs <X_RES> --ys <Y_RES> -i <INPUT_IMAGE> -o <OUTPUT_FILE> -m <MODE>

Options:
      --xs <X_RES>             Symbol resolution by X (number, 'auto', or 'auto%N' where N is percentage)
      --ys <Y_RES>             Symbol resolution by Y (number, 'auto', or 'auto%N' where N is percentage)
  -i <INPUT_IMAGE>             Input image file path
  -o <OUTPUT_FILE>             Output file path
  -m <MODE>                    Rendering mode [possible values: colorful, grayscale, inverted]
  -s, --symbols <SYMBOL_PACK>  Symbol pack to use for ASCII conversion [default: standard] [possible values: standard, detailed, blocks, minimal, digits, binary]
      --show-instantly         Print the ASCII art to console instantly
  -h, --help                   Print help
  -V, --version                Print version
```

# TO-DO
* FIX SQUARE IMAGES SQUISHING PROBLEM [ ]
* MORE IMAGES FORMATS TESTING [ ]
