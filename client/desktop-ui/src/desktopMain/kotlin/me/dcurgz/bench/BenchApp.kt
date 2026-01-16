package me.dcurgz.bench

import androidx.compose.runtime.Composable
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.*

fun main() = application {
    Window(
        onCloseRequest = ::exitApplication,
        title = "Bench",
        state = WindowState(width = 1200.dp, height = 800.dp)
    ) {
        BenchApp()
    }
}

@Composable
fun BenchApp(): String {
    return "Hello world"
}
