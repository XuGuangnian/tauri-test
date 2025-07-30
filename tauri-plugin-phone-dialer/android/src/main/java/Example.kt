package com.plugin.phonedialer

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.util.Log
import androidx.core.content.ContextCompat

/**
 * 拨号功能实现类
 */
class PhoneDialer(private val context: Context) {
    
    companion object {
        private const val TAG = "PhoneDialer"
    }
    
    /**
     * Ping 测试功能
     */
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
    
    /**
     * 拨打电话功能
     * @param phoneNumber 电话号码
     * @return 拨号结果
     */
    fun dialPhone(phoneNumber: String): PhoneDialResult {
        try {
            Log.d(TAG, "尝试拨打电话号码: $phoneNumber")
            
            // 验证电话号码
            if (phoneNumber.isBlank()) {
                return PhoneDialResult(
                    success = false,
                    message = "电话号码不能为空"
                )
            }
            
            // 清理电话号码（移除非数字字符，保留+号）
            val cleanedNumber = phoneNumber.filter { it.isDigit() || it == '+' }
            
            if (cleanedNumber.isEmpty()) {
                return PhoneDialResult(
                    success = false,
                    message = "无效的电话号码格式"
                )
            }
            
            // 检查拨号权限
            if (!hasCallPermission()) {
                return PhoneDialResult(
                    success = false,
                    message = "缺少拨号权限，请在应用设置中授予电话权限"
                )
            }
            
            // 执行拨号操作
            val success = performDialAction(cleanedNumber)
            
            return if (success) {
                PhoneDialResult(
                    success = true,
                    message = "成功发起拨号到 $cleanedNumber"
                )
            } else {
                PhoneDialResult(
                    success = false,
                    message = "拨号失败，请检查设备是否支持拨号功能"
                )
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "拨号过程中发生错误", e)
            return PhoneDialResult(
                success = false,
                message = "拨号失败: ${e.message}"
            )
        }
    }
    
    /**
     * 检查是否有拨号权限
     */
    private fun hasCallPermission(): Boolean {
        return ContextCompat.checkSelfPermission(
            context,
            Manifest.permission.CALL_PHONE
        ) == PackageManager.PERMISSION_GRANTED
    }
    
    /**
     * 执行拨号操作
     * @param phoneNumber 清理后的电话号码
     * @return 是否成功发起拨号
     */
    private fun performDialAction(phoneNumber: String): Boolean {
        return try {
            // 使用ACTION_CALL直接拨号（需要CALL_PHONE权限）
            val intent = Intent(Intent.ACTION_CALL).apply {
                data = Uri.parse("tel:$phoneNumber")
                flags = Intent.FLAG_ACTIVITY_NEW_TASK
            }
            
            // 检查是否有应用可以处理拨号Intent
            if (intent.resolveActivity(context.packageManager) != null) {
                context.startActivity(intent)
                Log.d(TAG, "成功发起拨号到: $phoneNumber")
                true
            } else {
                Log.w(TAG, "没有找到可以处理拨号的应用")
                // 如果ACTION_CALL失败，尝试使用ACTION_DIAL（不需要权限，但只是打开拨号界面）
                fallbackToDialer(phoneNumber)
            }
        } catch (e: Exception) {
            Log.e(TAG, "拨号失败，尝试备用方案", e)
            fallbackToDialer(phoneNumber)
        }
    }
    
    /**
     * 备用拨号方案：打开拨号界面
     * @param phoneNumber 电话号码
     * @return 是否成功打开拨号界面
     */
    private fun fallbackToDialer(phoneNumber: String): Boolean {
        return try {
            val intent = Intent(Intent.ACTION_DIAL).apply {
                data = Uri.parse("tel:$phoneNumber")
                flags = Intent.FLAG_ACTIVITY_NEW_TASK
            }
            
            if (intent.resolveActivity(context.packageManager) != null) {
                context.startActivity(intent)
                Log.d(TAG, "成功打开拨号界面: $phoneNumber")
                true
            } else {
                Log.e(TAG, "无法打开拨号界面")
                false
            }
        } catch (e: Exception) {
            Log.e(TAG, "打开拨号界面失败", e)
            false
        }
    }
}

/**
 * 拨号结果数据类
 */
data class PhoneDialResult(
    val success: Boolean,
    val message: String
)
