<template>
  <div class="tools-view">
    <h2 class="text-2xl font-bold mb-6">工具</h2>

    <div class="tools-container">
      <!-- 数据库解密 -->
      <div class="tool-card">
        <h3 class="tool-title">数据库解密</h3>
        <div class="tool-content">
          <div class="tool-input-group">
            <label>数据库路径</label>
            <input
              v-model="decryptDbPath"
              type="text"
              placeholder="请输入数据库路径"
              class="tool-input"
            />
          </div>
          <div class="tool-input-group">
            <label>密钥</label>
            <input
              v-model="decryptKey"
              type="text"
              placeholder="请输入解密密钥"
              class="tool-input"
            />
          </div>
          <div class="tool-input-group">
            <label>输出路径</label>
            <input
              v-model="decryptOutputPath"
              type="text"
              placeholder="请输入输出路径"
              class="tool-input"
            />
          </div>
          <button
            @click="decryptDatabase"
            :disabled="decrypting"
            class="tool-button"
          >
            {{ decrypting ? '解密中...' : '开始解密' }}
          </button>
          <div v-if="decryptResult" class="tool-result">
            {{ decryptResult }}
          </div>
        </div>
      </div>

      <!-- 数据库合并 -->
      <div class="tool-card">
        <h3 class="tool-title">数据库合并</h3>
        <div class="tool-content">
          <div class="tool-input-group">
            <label>源数据库路径（用逗号分隔）</label>
            <textarea
              v-model="mergeSourcePaths"
              placeholder="请输入源数据库路径，多个路径用逗号分隔"
              class="tool-textarea"
              rows="3"
            />
          </div>
          <div class="tool-input-group">
            <label>输出路径</label>
            <input
              v-model="mergeOutputPath"
              type="text"
              placeholder="请输入合并后的数据库路径"
              class="tool-input"
            />
          </div>
          <div class="tool-checkbox">
            <label>
              <input
                type="checkbox"
                v-model="removeDuplicates"
              />
              去除重复消息
            </label>
          </div>
          <button
            @click="mergeDatabases"
            :disabled="merging"
            class="tool-button"
          >
            {{ merging ? '合并中...' : '开始合并' }}
          </button>
          <div v-if="mergeResult" class="tool-result">
            {{ mergeResult }}
          </div>
        </div>
      </div>

      <!-- 获取微信信息 -->
      <div class="tool-card">
        <h3 class="tool-title">获取微信信息</h3>
        <div class="tool-content">
          <button
            @click="getWxInfo"
            :disabled="loadingWxInfo"
            class="tool-button"
          >
            {{ loadingWxInfo ? '获取中...' : '获取微信信息' }}
          </button>
          <div v-if="wxInfo" class="tool-result">
            <div v-for="info in wxInfo" :key="info.pid" class="wx-info-item">
              <div><strong>进程ID:</strong> {{ info.pid }}</div>
              <div><strong>版本:</strong> {{ info.version }}</div>
              <div><strong>账号:</strong> {{ info.account || 'N/A' }}</div>
              <div><strong>昵称:</strong> {{ info.nickname || 'N/A' }}</div>
              <div><strong>微信ID:</strong> {{ info.wxid || 'N/A' }}</div>
              <div><strong>密钥:</strong> {{ info.key ? info.key.substring(0, 16) + '...' : 'N/A' }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { wxApi } from '../api/index.js'

const decryptDbPath = ref('')
const decryptKey = ref('')
const decryptOutputPath = ref('')
const decrypting = ref(false)
const decryptResult = ref('')

const mergeSourcePaths = ref('')
const mergeOutputPath = ref('')
const removeDuplicates = ref(true)
const merging = ref(false)
const mergeResult = ref('')

const loadingWxInfo = ref(false)
const wxInfo = ref(null)

const decryptDatabase = async () => {
  if (!decryptDbPath.value || !decryptKey.value) {
    alert('请输入数据库路径和密钥')
    return
  }

  decrypting.value = true
  decryptResult.value = ''

  try {
    const response = await wxApi.decryptDb({
      db_path: decryptDbPath.value,
      key: decryptKey.value,
      output_path: decryptOutputPath.value || null
    })

    if (response.data.success) {
      decryptResult.value = `解密成功: ${response.data.message}`
    } else {
      decryptResult.value = `解密失败: ${response.data.message}`
    }
  } catch (error) {
    decryptResult.value = `解密失败: ${error.message}`
  } finally {
    decrypting.value = false
  }
}

const mergeDatabases = async () => {
  if (!mergeSourcePaths.value || !mergeOutputPath.value) {
    alert('请输入源数据库路径和输出路径')
    return
  }

  merging.value = true
  mergeResult.value = ''

  try {
    const sourcePaths = mergeSourcePaths.value
      .split(',')
      .map(p => p.trim())
      .filter(p => p)

    const response = await fetch('/api/db/merge', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        source_paths: sourcePaths,
        output_path: mergeOutputPath.value,
        remove_duplicates: removeDuplicates.value
      })
    })

    const result = await response.json()

    if (result.data?.success) {
      mergeResult.value = `合并成功: ${result.data.message}，共合并 ${result.data.total_inserted || 0} 条消息`
    } else {
      mergeResult.value = `合并失败: ${result.data?.message || result.message}`
    }
  } catch (error) {
    mergeResult.value = `合并失败: ${error.message}`
  } finally {
    merging.value = false
  }
}

const getWxInfo = async () => {
  loadingWxInfo.value = true
  wxInfo.value = null

  try {
    const response = await wxApi.getWxInfo()
    wxInfo.value = response.data
  } catch (error) {
    alert('获取微信信息失败: ' + error.message)
  } finally {
    loadingWxInfo.value = false
  }
}
</script>

<style scoped>
.tools-view {
  padding: 24px;
}

.tools-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 24px;
}

.tool-card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.tool-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #333;
}

.tool-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.tool-input-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.tool-input-group label {
  font-weight: 500;
  color: #333;
}

.tool-input,
.tool-textarea {
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.tool-textarea {
  resize: vertical;
  font-family: inherit;
}

.tool-checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tool-button {
  padding: 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
}

.tool-button:hover:not(:disabled) {
  background: #1976d2;
}

.tool-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.tool-result {
  padding: 12px;
  background: #f5f5f5;
  border-radius: 4px;
  font-size: 14px;
  word-break: break-all;
}

.wx-info-item {
  margin-bottom: 16px;
  padding: 12px;
  background: #f9f9f9;
  border-radius: 4px;
}

.wx-info-item div {
  margin-bottom: 8px;
}
</style>

