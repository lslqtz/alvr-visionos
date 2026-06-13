#pragma once
#import "ALVRClientCore/alvr_client_core.h"
#import "ShaderTypes.h"
#import <stdint.h>

static inline int64_t alvr_atomic_load_acquire(volatile int64_t *ptr) {
    return __atomic_load_n(ptr, __ATOMIC_ACQUIRE);
}

static inline void alvr_atomic_store_release(volatile int64_t *ptr, int64_t val) {
    __atomic_store_n(ptr, val, __ATOMIC_RELEASE);
}
