package dev.minimalium.engine

/**
 * Thin Android-facing wrapper around the Minimalium C ABI.
 *
 * The native library must be packaged by the consuming Android application.
 * This foundation bridge does not load or render web pages yet.
 */
class NativeEngine : AutoCloseable {
    private var handle: Long = nativeCreate()

    init {
        check(handle != 0L) { "Unable to create Minimalium native engine" }
    }

    /**
     * Validates and records an HTTP(S) target. This is not a network request.
     * @return 0 on success; nonzero on failure.
     */
    fun validateNavigationTarget(url: String): Int {
        check(handle != 0L) { "Engine is closed" }
        return nativeNavigate(handle, url)
    }

    override fun close() {
        if (handle != 0L) {
            nativeDestroy(handle)
            handle = 0L
        }
    }

    private external fun nativeCreate(): Long
    private external fun nativeNavigate(handle: Long, url: String): Int
    private external fun nativeDestroy(handle: Long)

    companion object {
        init {
            System.loadLibrary("minimalium_jni")
        }
    }
}
