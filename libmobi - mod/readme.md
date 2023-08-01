To build this library, all we need to is download libmobi-0.11

Inside of tools/
We need to add/modify the following files

Makefile.am - This file contains make instructions for the included tools. In our case, we stripped one of those tools and provide an easy API.
```make
# tools

# this lists the binaries to produce, the (non-PHONY, binary) targets in
# the previous manual Makefile
AM_CPPFLAGS = -I$(top_srcdir)/src -I$(top_builddir)/src


noinst_LIBRARIES = libmobitoolmod.a
libmobitoolmod_a_CFLAGS = $(ISO99_SOURCE) $(DEBUG_CFLAGS) -D_POSIX_C_SOURCE=200112L

libmobitoolmod_a_SOURCES = mobitoolmod.c common.c common.h $(top_builddir)/src/buffer.c $(top_builddir)/src/buffer.h $(top_builddir)/src/compression.c $(top_builddir)/src/compression.h $(top_builddir)/src/config.h $(top_builddir)/src/debug.c $(top_builddir)/src/debug.h $(top_builddir)/src/index.c $(top_builddir)/src/index.h $(top_builddir)/src/memory.c $(top_builddir)/src/memory.h $(top_builddir)/src/meta.c $(top_builddir)/src/meta.h $(top_builddir)/src/mobi.h $(top_builddir)/src/parse_rawml.c $(top_builddir)/src/parse_rawml.h $(top_builddir)/src/read.c $(top_builddir)/src/read.h $(top_builddir)/src/structure.c $(top_builddir)/src/structure.h $(top_builddir)/src/util.c $(top_builddir)/src/util.h $(top_builddir)/src/write.c $(top_builddir)/src/write.h $(top_builddir)/src/xmlwriter.c $(top_builddir)/src/xmlwriter.h $(top_builddir)/src/encryption.h $(top_builddir)/src/encryption.c $(top_builddir)/src/sha1.c $(top_builddir)/src/sha1.h $(top_builddir)/src/sha1.c $(top_builddir)/src/randombytes.h $(top_builddir)/src/sha1.c $(top_builddir)/src/randombytes.c $(top_builddir)/src/miniz.c $(top_builddir)/src/miniz.h $(top_builddir)/src/opf.c $(top_builddir)/src/opf.c

```

mobitoolmod.c - Simply remove the `main` method and add this method in it's place.

```c
int convertToEpub(char *sourceFilePath, char *destFolderPath) {

    char sourcePath[FILENAME_MAX];
    strncpy(sourcePath, sourceFilePath, FILENAME_MAX - 1);
    strncpy(outdir, destFolderPath, FILENAME_MAX - 1);

    normalize_path(outdir);
    normalize_path(sourcePath);
    create_epub_opt = true;
    outdir_opt = true;

    if (!dir_exists(outdir)) {
        printf("Output directory is not valid\n");
        return ERROR;
    }

    size_t outdir_length = strlen(outdir);

    if (outdir[outdir_length - 1] != separator) {
        // append separator
        if (outdir_length >= FILENAME_MAX - 2) {
            printf("Output directory name too long\n");
            return ERROR;
        }
        outdir[outdir_length++] = separator;
        outdir[outdir_length] = '\0';
    }

    sourcePath[FILENAME_MAX - 1] = '\0';
    int ret = SUCCESS;
    ret = loadfilename(sourcePath);

    return ret;

}
```

Also make sure to include the h file in the heading of this file - This contains the standard export methods of the other header files.
```c
#include "./mobitoolmod.h"
```


Finally, let's add the header file

```h
/** @brief Visibility attributes for symbol export */
#if defined (__CYGWIN__) || defined (__MINGW32__)
#define API_EXPORT __attribute__((visibility("default"))) __declspec(dllexport) extern
#elif defined (_WIN32)
#define API_EXPORT __declspec(dllexport)
#else
#define API_EXPORT __attribute__((__visibility__("default")))
#endif



#ifdef __cplusplus
extern "C"
{
#endif
API_EXPORT int convertToEpub(char *sourceFilePath, char *destFolderPath);


#ifdef __cplusplus
}
#endif

```


To compile this project as a static library, run the following in the root directory. We are using the built in libxml2 and zlib implementations since we do not want any dynamic linking.

Linux:
```
./autogen.sh && ./configure --with-libxml2=no --with-zlib=no && make
```

Windows - Ensure that mingw is installed on system before compiling:
```
./autogen.sh && ./configure --host x86_64-w64-mingw32 --with-libxml2=no --with-zlib=no && make
```
Note: For windows, ensure when you are linking the project, you include the flag 
```
link-arg=-Wl,--allow-multiple-definition
```
Also ensure that you are linking using mingw, otherwise you will face errors such as
"unresolved external symbol __mingw_vfprintf "
and
"undefined reference to `localtime'"

IMPORTANT: Compiling results in some sort of garbage files being generated. This can actually corrupt future compilations. At the moment, I suggest erasing the directory between each compile.

Links:
https://stackoverflow.com/questions/25105573/compile-for-windows-using-mingw-w64-and-autotools - For general autotool building instructions

