
#include <mobitoolmod.h>

int main(int argc, char *argv[]) {
    if (argc != 3) {
    printf("Usage: %s entrysrc outdir\n", argv[0]);
    return 1;
    }

    return convertToEpub(argv[1], argv[2]);
    // printf("Entry directory: %s\n", entrydir);
    // printf("Output directory: %s\n", outdir);

    return 0;
    
}
