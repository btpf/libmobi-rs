Put `libmobitoolmod.a` and `mobitoolmod.h` inside this folder and proceed with below directions.


To Compile and test, i recommend the following commands

For testing the static library:
```
gcc -static ./test.c libmobitoolmod.a -I/home/<FOLDER TO .h DEFINITIONS>
```


For dynamic library testing
```
g++ test.c -I/home/<PATH TO HEADER FILE> -L/home/<PATH TO .so FILE> -lmobitoolmod
LD_LIBRARY_PATH=./<Folder containing .so or .dll dependency>/ ./a.out
```
