#pragma once

#include <objc/message.h>
#include <objc/objc.h>
#include <objc/runtime.h>

template <typename R, typename... Args>
static inline R send(id self, SEL sel, Args... args) {
    return reinterpret_cast<R (*)(id, SEL, Args...)>(objc_msgSend)(self, sel,
                                                                   args...);
}
