#ifndef TLSH_CAPI
#define TLSH_CAPI

#ifdef __cplusplus
extern "C" {
#endif
    void* tlsh_new();
    void tlsh_delete(void* tlsh_instance);
    void tlsh_update(void* tlsh_instance, const unsigned char* data, unsigned int len);
    void tlsh_final(void* tlsh_instance);
    void tlsh_reset(void* tlsh_instance);
    const char* tlsh_getHash(void* tlsh_instance);
    const char* tlsh_getHashBuf(void* tlsh_instance, char* buffer, unsigned int buffer_len);
    int tlsh_totalDiff(void* tlsh_instance, void* tlsh_instance2);
    int tlsh_fromTlshStr(void* tlsh_instance, const char* tlsh_str);
#ifdef __cplusplus
};
#endif



#endif