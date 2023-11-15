#include "tlsh_capi.h"
#include "tlsh.h"

#ifdef __cplusplus
extern "C" {
    void* tlsh_new() {
        return (void*) new Tlsh();
    }

    void tlsh_delete(void* tlsh_instance) {
        delete (Tlsh*)tlsh_instance;
    }

    void tlsh_update(void* tlsh_instance, const unsigned char* data, unsigned int len) {
        ((Tlsh*)tlsh_instance)->update(data, len);
    }

    void tlsh_final(void* tlsh_instance) {
        ((Tlsh*)tlsh_instance)->final();
    }

    void tlsh_reset(void* tlsh_instance) {
        ((Tlsh*)tlsh_instance)->reset();
    }

    const char* tlsh_getHash(void* tlsh_instance) {
        return ((Tlsh*)tlsh_instance)->getHash();
    }

    const char* tlsh_getHashBuf(void* tlsh_instance, char* buffer, unsigned int buffer_len) {
        return ((Tlsh*)tlsh_instance)->getHash(buffer, buffer_len);
    }

    int totalDiff(void* tlsh_instance, void* tlsh_instance2) {
        return ((Tlsh*)tlsh_instance)->totalDiff((Tlsh*)tlsh_instance2);
    }

    int fromTlshStr(void* tlsh_instance, const char* tlsh_str) {
        return ((Tlsh*)tlsh_instance)->fromTlshStr(tlsh_str);
    }

}

#else
#error Cannot build TLSH without C++ compiler
#endif
