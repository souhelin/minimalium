#include <jni.h>
#include <cstdint>
#include "minimalium.h"

extern "C" JNIEXPORT jlong JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeCreate(JNIEnv *, jobject) {
    return reinterpret_cast<jlong>(minimalium_engine_create());
}

extern "C" JNIEXPORT jint JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeNavigate(
    JNIEnv *env, jobject, jlong handle, jstring url) {
    if (handle == 0 || url == nullptr) {
        return 1;
    }

    const char *target = env->GetStringUTFChars(url, nullptr);
    if (target == nullptr) {
        return 2;
    }

    const auto status = minimalium_engine_navigate(
        reinterpret_cast<MinimaliumEngine *>(handle), target);
    env->ReleaseStringUTFChars(url, target);
    return static_cast<jint>(status);
}

extern "C" JNIEXPORT void JNICALL
Java_dev_minimalium_engine_NativeEngine_nativeDestroy(
    JNIEnv *, jobject, jlong handle) {
    if (handle != 0) {
        minimalium_engine_destroy(reinterpret_cast<MinimaliumEngine *>(handle));
    }
}
