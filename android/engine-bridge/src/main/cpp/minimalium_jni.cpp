#include <jni.h>
#include <algorithm>
#include <vector>
#include "minimalium.h"

extern "C" JNIEXPORT jlong JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeCreate(JNIEnv *, jobject) {
    return reinterpret_cast<jlong>(minimalium_engine_create());
}

extern "C" JNIEXPORT jint JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeNavigate(
    JNIEnv *env, jobject, jlong handle, jbyteArray url) {
    if (handle == 0 || url == nullptr) {
        return 1;
    }

    const jsize length = env->GetArrayLength(url);
    std::vector<jbyte> target(static_cast<size_t>(length) + 1, 0);
    if (length > 0) {
        env->GetByteArrayRegion(url, 0, length, target.data());
        if (env->ExceptionCheck()) {
            return 2;
        }
    }

    if (std::find(target.begin(), target.end() - 1, 0) != target.end() - 1) {
        return 3;
    }

    const auto status = minimalium_engine_navigate(
        reinterpret_cast<MinimaliumEngine *>(handle),
        reinterpret_cast<const char *>(target.data()));
    return static_cast<jint>(status);
}

extern "C" JNIEXPORT void JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeDestroy(
    JNIEnv *, jobject, jlong handle) {
    if (handle != 0) {
        minimalium_engine_destroy(reinterpret_cast<MinimaliumEngine *>(handle));
    }
}
