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