/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package testapp

import uniffi.zcash.*

class App {
    fun callAmount() {
        val amount = ZcashAmount(100)
        amount.value()
    }
}

fun main() {
    val app = App()
    app.callAmount()
    println("Kotlin test application succesfully executed ✅")
}
