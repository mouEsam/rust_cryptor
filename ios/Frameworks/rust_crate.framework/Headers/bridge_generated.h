#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_CryptorHandle {
  int64_t field0;
} wire_CryptorHandle;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_greet(int64_t port_);

void wire_cryptor_new(int64_t port_,
                      int64_t sub_pool_id,
                      struct wire_uint_8_list *key,
                      uintptr_t iv_length);

void wire_cryptor_encrypt(int64_t port_,
                          struct wire_CryptorHandle *cryptor,
                          struct wire_uint_8_list *text);

void wire_cryptor_decrypt(int64_t port_,
                          struct wire_CryptorHandle *cryptor,
                          struct wire_uint_8_list *data);

void wire_cryptor_remove(int64_t port_, struct wire_CryptorHandle *cryptor);

struct wire_CryptorHandle *new_box_autoadd_cryptor_handle(void);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_greet);
    dummy_var ^= ((int64_t) (void*) wire_cryptor_new);
    dummy_var ^= ((int64_t) (void*) wire_cryptor_encrypt);
    dummy_var ^= ((int64_t) (void*) wire_cryptor_decrypt);
    dummy_var ^= ((int64_t) (void*) wire_cryptor_remove);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cryptor_handle);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}