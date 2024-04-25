# maestro-test

`maestro-test` is a test suite for [Maestro](https://github.com/llenotre/maestro).

It is meant to test each feature of the kernel to ensure no regression has been made. It may be run by continuous integration.

The test suite reports its results through the serial port.



## Build an image

To use the test suite, one must first build a disk image.

This can be done with the `build.sh` script:
```sh
./build.sh
```

This script produces the `disk` file, which can then be used by QEMU with the following option:
```
-drive file=disk,format=raw
```
