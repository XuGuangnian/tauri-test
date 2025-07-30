<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import {
  ping,
  dialPhoneNumber,
  requestPhonePermission,
  checkPhonePermission,
  type DialPhoneResult,
  type PermissionResult,
} from "tauri-plugin-phone-dialer-api";

// 条件导入相机插件，只在需要时导入
let takePicture: any = null;
let recordVideo: any = null;

const greetMsg = ref("");
const name = ref("");
const pictureResult = ref<{
  imageData: string;
  width: number;
  height: number;
} | null>(null);
const videoResult = ref<{
  videoData: string;
  width: number;
  height: number;
} | null>(null);
const cameraStatus = ref("");
const isAndroid = ref(false);
const platformName = ref("");

// Phone Dialer 插件测试相关
const pingInput = ref("Hello from TypeScript!");
const pingResult = ref("");
const rustPingInput = ref("Hello from Rust Command!");
const rustPingResult = ref("");

// 拨号功能相关
const phoneNumber = ref("10086");
const dialResult = ref("");
const rustDialPhoneNumber = ref("10010");
const rustDialResult = ref("");

// 权限相关
const permissionStatus = ref("");
const hasPhonePermission = ref(false);

onMounted(async () => {
  try {
    platformName.value = await platform();
    isAndroid.value = platformName.value === "android";

    // 只在 Android 平台上动态导入相机插件
    if (isAndroid.value) {
      try {
        const cameraModule = await import("tauri-plugin-camera");
        takePicture = cameraModule.takePicture;
        recordVideo = cameraModule.recordVideo;
        cameraStatus.value = "相机插件已加载，可以使用拍照和录制功能";
      } catch (error) {
        cameraStatus.value = "相机插件加载失败";
        console.error("相机插件加载错误:", error);
      }

      // 检查电话权限状态
      try {
        await checkPermissionStatus();
      } catch (error) {
        console.error("权限检查失败:", error);
        permissionStatus.value = "权限检查失败";
      }
    }
  } catch (error) {
    console.error("平台检测错误:", error);
    platformName.value = "未知";
  }
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function takePhoto() {
  if (!isAndroid.value) {
    cameraStatus.value = "相机功能仅在 Android 平台上可用";
    return;
  }

  if (!takePicture) {
    cameraStatus.value = "相机插件未正确加载";
    return;
  }

  try {
    cameraStatus.value = "正在拍照...";
    const response = await takePicture();
    pictureResult.value = response;
    cameraStatus.value = `拍照成功！图片尺寸: ${response.width}x${response.height}`;
  } catch (error) {
    cameraStatus.value = `拍照失败: ${error}`;
    console.error("拍照错误:", error);
  }
}

async function recordVideoFunc() {
  if (!isAndroid.value) {
    cameraStatus.value = "录制功能仅在 Android 平台上可用";
    return;
  }

  if (!recordVideo) {
    cameraStatus.value = "相机插件未正确加载";
    return;
  }

  try {
    cameraStatus.value = "正在录制视频...";
    const response = await recordVideo();
    videoResult.value = response;
    cameraStatus.value = `录制成功！视频尺寸: ${response.width}x${response.height}`;
  } catch (error) {
    cameraStatus.value = `录制失败: ${error}`;
    console.error("录制错误:", error);
  }
}

function clearResults() {
  pictureResult.value = null;
  videoResult.value = null;
  cameraStatus.value = "";
}

// Phone Dialer 插件测试函数
async function testTypeScriptPing() {
  try {
    pingResult.value = "正在调用 TypeScript ping...";
    const result = await ping(pingInput.value);
    pingResult.value = `TypeScript ping 成功: ${result}`;
  } catch (error) {
    pingResult.value = `TypeScript ping 失败: ${error}`;
    console.error("TypeScript ping 错误:", error);
  }
}

async function testRustPing() {
  try {
    rustPingResult.value = "正在调用 Rust command ping...";
    const result = await invoke("plugin:phone-dialer|ping", {
      payload: { value: rustPingInput.value },
    });
    rustPingResult.value = `Rust ping 成功: ${JSON.stringify(result)}`;
  } catch (error) {
    rustPingResult.value = `Rust ping 失败: ${error}`;
    console.error("Rust ping 错误:", error);
  }
}

function clearPingResults() {
  pingResult.value = "";
  rustPingResult.value = "";
}

// 拨号功能测试函数
async function testDialPhone() {
  try {
    dialResult.value = "正在调用 TypeScript 拨号...";
    const result: DialPhoneResult = await dialPhoneNumber(phoneNumber.value);
    dialResult.value = `TypeScript 拨号结果: ${
      result.success ? "成功" : "失败"
    } - ${result.message}`;
  } catch (error) {
    dialResult.value = `TypeScript 拨号失败: ${error}`;
    console.error("TypeScript 拨号错误:", error);
  }
}

async function testRustDialPhone() {
  try {
    rustDialResult.value = "正在调用 Rust command 拨号...";
    const result = await invoke("plugin:phone-dialer|dial_phone_number", {
      payload: { phoneNumber: rustDialPhoneNumber.value },
    });
    rustDialResult.value = `Rust 拨号结果: ${JSON.stringify(result)}`;
  } catch (error) {
    rustDialResult.value = `Rust 拨号失败: ${error}`;
    console.error("Rust 拨号错误:", error);
  }
}

function clearDialResults() {
  dialResult.value = "";
  rustDialResult.value = "";
}

// 权限相关函数
async function checkPermissionStatus() {
  try {
    const result: PermissionResult = await checkPhonePermission();
    hasPhonePermission.value = result.success;
    permissionStatus.value = result.message;
  } catch (error) {
    console.error("检查权限状态失败:", error);
    permissionStatus.value = "权限检查失败";
    hasPhonePermission.value = false;
  }
}

async function requestPermission() {
  try {
    permissionStatus.value = "正在请求电话权限...";
    const result: PermissionResult = await requestPhonePermission();
    permissionStatus.value = result.message;

    // 请求后重新检查权限状态
    setTimeout(async () => {
      await checkPermissionStatus();
    }, 1000);
  } catch (error) {
    console.error("请求权限失败:", error);
    permissionStatus.value = "权限请求失败";
  }
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <!-- 原有的问候功能 -->
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>

    <!-- 相机功能测试 -->
    <div class="camera-section">
      <h2>📱 相机功能测试</h2>

      <!-- 平台信息显示 -->
      <div class="platform-info">
        <p><strong>当前平台:</strong> {{ platformName || "检测中..." }}</p>
        <p v-if="!isAndroid" class="platform-warning">
          ⚠️ 相机插件仅在 Android 平台上可用。当前平台为: {{ platformName }}
        </p>
        <p v-else class="platform-success">
          ✅ Android 平台已检测到，相机功能可用
        </p>
      </div>

      <div class="camera-controls">
        <button
          @click="takePhoto"
          class="camera-btn"
          :disabled="!isAndroid"
          :class="{ disabled: !isAndroid }"
        >
          📸 拍照
        </button>
        <button
          @click="recordVideoFunc"
          class="camera-btn"
          :disabled="!isAndroid"
          :class="{ disabled: !isAndroid }"
        >
          🎥 录制视频
        </button>
        <button @click="clearResults" class="clear-btn">🗑️ 清除结果</button>
      </div>

      <!-- 状态显示 -->
      <div v-if="cameraStatus" class="status">
        {{ cameraStatus }}
      </div>

      <!-- 拍照结果 -->
      <div v-if="pictureResult" class="result-section">
        <h3>📸 拍照结果:</h3>
        <p>尺寸: {{ pictureResult.width }} x {{ pictureResult.height }}</p>
        <div class="image-preview">
          <img
            :src="`data:image/jpeg;base64,${pictureResult.imageData}`"
            alt="拍摄的照片"
            class="preview-image"
          />
        </div>
      </div>

      <!-- 录制视频结果 -->
      <div v-if="videoResult" class="result-section">
        <h3>🎥 录制视频结果:</h3>
        <p>尺寸: {{ videoResult.width }} x {{ videoResult.height }}</p>
        <div class="video-preview">
          <video
            :src="`data:video/mp4;base64,${videoResult.videoData}`"
            controls
            class="preview-video"
          >
            您的浏览器不支持视频播放
          </video>
        </div>
      </div>

      <!-- 使用说明 -->
      <div class="instructions">
        <h3>📋 使用说明:</h3>
        <ul>
          <li>此插件仅在 Android 平台上工作</li>
          <li>需要在实际的 Android 设备上测试</li>
          <li>确保应用已获得相机权限</li>
          <li>点击"拍照"会打开相机进行拍照</li>
          <li>点击"录制视频"会打开相机进行视频录制</li>
        </ul>
      </div>
    </div>

    <!-- Phone Dialer 插件测试 -->
    <div class="phone-dialer-section">
      <h2>📞 Phone Dialer 插件测试</h2>

      <div class="plugin-info">
        <p>
          <strong>插件状态:</strong> Phone Dialer 插件已加载，可以测试 ping 功能
        </p>
        <p class="plugin-success">✅ 此插件在所有平台上都可以使用</p>
      </div>

      <!-- 权限状态显示 -->
      <div v-if="isAndroid" class="permission-info">
        <h3>📱 电话权限状态</h3>
        <div class="permission-status">
          <span
            :class="{
              'permission-granted': hasPhonePermission,
              'permission-denied': !hasPhonePermission,
            }"
          >
            {{ hasPhonePermission ? "✅ 已授权" : "❌ 未授权" }}
          </span>
          <p>{{ permissionStatus }}</p>
        </div>
        <div v-if="!hasPhonePermission" class="permission-actions">
          <button @click="requestPermission" class="permission-btn">
            🔓 请求电话权限
          </button>
          <button
            @click="checkPermissionStatus"
            class="permission-btn check-btn"
          >
            🔍 重新检查权限
          </button>
        </div>
      </div>

      <!-- TypeScript Ping 测试 -->
      <div class="ping-test-section">
        <h3>🔷 TypeScript Ping 测试</h3>
        <div class="ping-controls">
          <input
            v-model="pingInput"
            placeholder="输入要发送的消息..."
            class="ping-input"
          />
          <button @click="testTypeScriptPing" class="ping-btn ts-btn">
            📡 TS Ping
          </button>
        </div>
        <div v-if="pingResult" class="ping-result">
          {{ pingResult }}
        </div>
      </div>

      <!-- Rust Command Ping 测试 -->
      <div class="ping-test-section">
        <h3>🦀 Rust Command Ping 测试</h3>
        <div class="ping-controls">
          <input
            v-model="rustPingInput"
            placeholder="输入要发送的消息..."
            class="ping-input"
          />
          <button @click="testRustPing" class="ping-btn rust-btn">
            ⚡ Rust Ping
          </button>
        </div>
        <div v-if="rustPingResult" class="ping-result">
          {{ rustPingResult }}
        </div>
      </div>

      <div class="ping-controls">
        <button @click="clearPingResults" class="clear-btn">🗑️ 清除结果</button>
      </div>

      <!-- 拨号功能测试 -->
      <div class="dial-section">
        <h2>📞 拨号功能测试</h2>

        <!-- TypeScript 拨号测试 -->
        <div class="ping-test-section">
          <h3>🔷 TypeScript 拨号测试</h3>
          <div class="ping-controls">
            <input
              v-model="phoneNumber"
              placeholder="输入电话号码 (如: 10086)"
              class="ping-input"
            />
            <button @click="testDialPhone" class="ping-btn ts-btn">
              📞 TS 拨号
            </button>
          </div>
          <div v-if="dialResult" class="ping-result">
            {{ dialResult }}
          </div>
        </div>

        <!-- Rust Command 拨号测试 -->
        <div class="ping-test-section">
          <h3>🦀 Rust Command 拨号测试</h3>
          <div class="ping-controls">
            <input
              v-model="rustDialPhoneNumber"
              placeholder="输入电话号码 (如: 10010)"
              class="ping-input"
            />
            <button @click="testRustDialPhone" class="ping-btn rust-btn">
              📞 Rust 拨号
            </button>
          </div>
          <div v-if="rustDialResult" class="ping-result">
            {{ rustDialResult }}
          </div>
        </div>

        <div class="ping-controls">
          <button @click="clearDialResults" class="clear-btn">
            🗑️ 清除拨号结果
          </button>
        </div>
      </div>

      <!-- 使用说明 -->
      <div class="instructions">
        <h3>📋 使用说明:</h3>
        <ul>
          <li>
            <strong>TypeScript Ping:</strong> 使用插件的 TypeScript API 直接调用
          </li>
          <li>
            <strong>Rust Command Ping:</strong> 使用 Tauri 的 invoke 方法调用
            Rust 命令
          </li>
          <li>两种方式都会调用相同的 Rust 后端逻辑</li>
          <li>可以输入任意文本进行测试</li>
          <li>观察返回结果的格式差异</li>
          <li><strong>拨号功能:</strong> 在 Android 平台上可以实际拨打电话</li>
          <li>
            <strong>注意:</strong>
            拨号功能需要电话权限，首次使用时系统会提示授权
          </li>
          <li>
            <strong>测试号码:</strong> 建议使用客服号码如 10086、10010
            等进行测试
          </li>
        </ul>
      </div>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

/* 相机功能样式 */
.camera-section {
  margin-top: 2rem;
  padding: 1.5rem;
  border: 1px solid #ddd;
  border-radius: 12px;
  background-color: #fafafa;
}

.platform-info {
  background-color: #f8f9fa;
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  border-left: 4px solid #007bff;
}

.platform-warning {
  color: #856404;
  background-color: #fff3cd;
  padding: 0.75rem;
  border-radius: 4px;
  border-left: 4px solid #ffc107;
  margin: 0.5rem 0;
}

.platform-success {
  color: #155724;
  background-color: #d4edda;
  padding: 0.75rem;
  border-radius: 4px;
  border-left: 4px solid #28a745;
  margin: 0.5rem 0;
}

.camera-controls {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin: 1rem 0;
  flex-wrap: wrap;
}

.camera-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 120px;
}

.camera-btn:hover:not(.disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.camera-btn.disabled {
  background: #6c757d;
  cursor: not-allowed;
  opacity: 0.6;
}

.camera-btn:disabled {
  background: #6c757d;
  cursor: not-allowed;
  opacity: 0.6;
}

.clear-btn {
  background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
  color: white;
}

.status {
  text-align: center;
  padding: 1rem;
  margin: 1rem 0;
  background-color: #e8f4fd;
  border-radius: 8px;
  font-weight: 500;
  color: #2c3e50;
}

.result-section {
  margin: 1.5rem 0;
  padding: 1rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.result-section h3 {
  margin-top: 0;
  color: #2c3e50;
}

.preview-image,
.preview-video {
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.image-preview,
.video-preview {
  text-align: center;
  margin-top: 1rem;
}

.instructions {
  margin-top: 1.5rem;
  padding: 1rem;
  background-color: #fff3cd;
  border-radius: 8px;
  border-left: 4px solid #ffc107;
}

.instructions h3 {
  margin-top: 0;
  color: #856404;
}

.instructions ul {
  text-align: left;
  color: #856404;
  margin: 0.5rem 0;
}

.instructions li {
  margin: 0.5rem 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }

  /* 暗色主题下的相机功能样式 */
  .camera-section {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .platform-info {
    background-color: #2d3748;
    border-left-color: #4299e1;
  }

  .platform-warning {
    background-color: #2d3748;
    color: #f6e05e;
    border-left-color: #f6e05e;
  }

  .platform-success {
    background-color: #2d3748;
    color: #68d391;
    border-left-color: #68d391;
  }

  .status {
    background-color: #2d3748;
    color: #e2e8f0;
  }

  .result-section {
    background-color: #2d3748;
    color: #e2e8f0;
  }

  .result-section h3 {
    color: #e2e8f0;
  }

  .instructions {
    background-color: #2d3748;
    border-left-color: #f6e05e;
  }

  .instructions h3 {
    color: #f6e05e;
  }

  .instructions ul {
    color: #e2e8f0;
  }
}

/* Phone Dialer 插件测试样式 */
.phone-dialer-section {
  margin-top: 2rem;
  padding: 1.5rem;
  border: 1px solid #ddd;
  border-radius: 12px;
  background-color: #fafafa;
}

.plugin-info {
  background-color: #f8f9fa;
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  border-left: 4px solid #007bff;
}

.plugin-success {
  color: #155724;
  background-color: #d4edda;
  padding: 0.75rem;
  border-radius: 4px;
  border-left: 4px solid #28a745;
  margin: 0.5rem 0;
}

.ping-test-section {
  margin: 1.5rem 0;
  padding: 1rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.ping-test-section h3 {
  margin-top: 0;
  color: #2c3e50;
}

.ping-controls {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin: 1rem 0;
  flex-wrap: wrap;
}

.ping-input {
  flex: 1;
  min-width: 200px;
  padding: 0.6em 1.2em;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 1em;
  background-color: #ffffff;
}

.ping-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 120px;
}

.ping-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.ts-btn {
  background: linear-gradient(135deg, #3b82f6 0%, #1e40af 100%);
}

.rust-btn {
  background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
}

.ping-result {
  text-align: center;
  padding: 1rem;
  margin: 1rem 0;
  background-color: #e8f4fd;
  border-radius: 8px;
  font-weight: 500;
  color: #2c3e50;
  border-left: 4px solid #3b82f6;
}

.dial-section {
  margin-top: 2rem;
  padding: 1.5rem;
  border: 1px solid #ddd;
  border-radius: 12px;
  background-color: #fafafa;
}

/* 权限相关样式 */
.permission-info {
  background-color: #f8f9fa;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 1.5rem 0;
  border-left: 4px solid #007bff;
}

.permission-info h3 {
  margin-top: 0;
  color: #2c3e50;
}

.permission-status {
  margin: 1rem 0;
  text-align: center;
}

.permission-granted {
  color: #28a745;
  font-weight: bold;
  font-size: 1.1em;
}

.permission-denied {
  color: #dc3545;
  font-weight: bold;
  font-size: 1.1em;
}

.permission-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-top: 1rem;
  flex-wrap: wrap;
}

.permission-btn {
  background: linear-gradient(135deg, #28a745 0%, #20c997 100%);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 140px;
}

.permission-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.check-btn {
  background: linear-gradient(135deg, #17a2b8 0%, #138496 100%);
}

@media (prefers-color-scheme: dark) {
  .phone-dialer-section {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .plugin-info {
    background-color: #2d3748;
    border-left-color: #4299e1;
  }

  .plugin-success {
    background-color: #2d3748;
    color: #68d391;
    border-left-color: #68d391;
  }

  .ping-test-section {
    background-color: #2d3748;
    color: #e2e8f0;
  }

  .ping-test-section h3 {
    color: #e2e8f0;
  }

  .ping-input {
    background-color: #4a5568;
    border-color: #6b7280;
    color: #e2e8f0;
  }

  .ping-result {
    background-color: #2d3748;
    color: #e2e8f0;
    border-left-color: #4299e1;
  }

  .dial-section {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .permission-info {
    background-color: #2d3748;
    border-left-color: #4299e1;
  }

  .permission-info h3 {
    color: #e2e8f0;
  }

  .permission-granted {
    color: #68d391;
  }

  .permission-denied {
    color: #f56565;
  }
}
</style>
