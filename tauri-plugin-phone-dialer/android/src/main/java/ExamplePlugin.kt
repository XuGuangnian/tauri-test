package com.plugin.phonedialer

import android.app.Activity
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.Manifest

@InvokeArg
class PingArgs {
  var value: String? = null
}

@InvokeArg
class DialPhoneArgs {
  var phoneNumber: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = PhoneDialer(activity)
    private val PERMISSION_REQUEST_CODE = 1001

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun dialPhoneNumber(invoke: Invoke) {
        val args = invoke.parseArgs(DialPhoneArgs::class.java)
        val phoneNumber = args.phoneNumber ?: ""

        // 检查权限
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.CALL_PHONE) 
            != PackageManager.PERMISSION_GRANTED) {
            
            // 请求权限
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.CALL_PHONE),
                PERMISSION_REQUEST_CODE
            )
            
            val ret = JSObject()
            ret.put("success", false)
            ret.put("message", "正在请求拨号权限，请在弹出的对话框中允许权限，然后重试")
            invoke.resolve(ret)
            return
        }

        val result = implementation.dialPhone(phoneNumber)

        val ret = JSObject()
        ret.put("success", result.success)
        ret.put("message", result.message)
        invoke.resolve(ret)
    }

    @Command
    fun requestPhonePermission(invoke: Invoke) {
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.CALL_PHONE) 
            != PackageManager.PERMISSION_GRANTED) {
            
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.CALL_PHONE),
                PERMISSION_REQUEST_CODE
            )
            
            val ret = JSObject()
            ret.put("success", false)
            ret.put("message", "权限请求已发送，请在弹出的对话框中允许权限")
            invoke.resolve(ret)
        } else {
            val ret = JSObject()
            ret.put("success", true)
            ret.put("message", "已具有拨号权限")
            invoke.resolve(ret)
        }
    }

    @Command
    fun checkPhonePermission(invoke: Invoke) {
        val hasPermission = ContextCompat.checkSelfPermission(activity, Manifest.permission.CALL_PHONE) == PackageManager.PERMISSION_GRANTED
        
        val ret = JSObject()
        ret.put("success", hasPermission)
        ret.put("message", if (hasPermission) "已具有拨号权限" else "缺少拨号权限")
        invoke.resolve(ret)
    }
}
