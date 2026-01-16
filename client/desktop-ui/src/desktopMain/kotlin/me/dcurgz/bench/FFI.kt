object FFI {
    init {
        System.loadLibrary("bench_core")
    }
    external fun sendMessage(text: String): String
}