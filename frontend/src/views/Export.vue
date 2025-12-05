<template>
  <div class="export-view">
    <h2 class="text-2xl font-bold mb-6">数据导出</h2>

    <div class="export-container">
      <!-- 导出配置 -->
      <div class="export-config">
        <div class="config-section">
          <label class="config-label">数据库路径</label>
          <input
            v-model="mergePath"
            type="text"
            placeholder="请输入合并后的数据库路径"
            class="config-input"
          />
        </div>

        <div class="config-section">
          <label class="config-label">联系人（可选）</label>
          <input
            v-model="wxid"
            type="text"
            placeholder="留空则导出所有联系人"
            class="config-input"
          />
        </div>

        <div class="config-section">
          <label class="config-label">时间范围（可选）</label>
          <div class="time-range">
            <input
              v-model="startTime"
              type="datetime-local"
              class="config-input"
              placeholder="开始时间"
            />
            <span class="time-separator">至</span>
            <input
              v-model="endTime"
              type="datetime-local"
              class="config-input"
              placeholder="结束时间"
            />
          </div>
        </div>

        <div class="config-section">
          <label class="config-label">导出格式</label>
          <div class="format-options">
            <label
              v-for="format in exportFormats"
              :key="format.value"
              class="format-option"
            >
              <input
                type="radio"
                v-model="selectedFormat"
                :value="format.value"
              />
              <span>{{ format.label }}</span>
            </label>
          </div>
        </div>

        <div class="config-section">
          <label class="config-label">输出路径（可选）</label>
          <input
            v-model="outputPath"
            type="text"
            placeholder="留空则自动生成文件名"
            class="config-input"
          />
        </div>

        <button
          @click="startExport"
          :disabled="loading || !mergePath"
          class="export-button"
        >
          {{ loading ? '导出中...' : '开始导出' }}
        </button>
      </div>

      <!-- 导出进度 -->
      <div v-if="exporting" class="export-progress">
        <div class="progress-header">
          <h3>导出进度</h3>
          <span class="progress-status">{{ progressMessage }}</span>
        </div>
        <div v-if="progress > 0" class="progress-bar-container">
          <div class="progress-bar" :style="{ width: progress + '%' }"></div>
        </div>
      </div>

      <!-- 导出结果 -->
      <div v-if="exportResult" class="export-result">
        <div :class="['result-message', exportResult.success ? 'success' : 'error']">
          {{ exportResult.message }}
        </div>
        <div v-if="exportResult.success && exportResult.file_path" class="result-file">
          <span>文件路径：</span>
          <a :href="exportResult.file_path" target="_blank" class="file-link">
            {{ exportResult.file_path }}
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { chatApi } from '../api/index.js'

const mergePath = ref('')
const wxid = ref('')
const startTime = ref('')
const endTime = ref('')
const selectedFormat = ref('csv')
const outputPath = ref('')
const loading = ref(false)
const exporting = ref(false)
const progress = ref(0)
const progressMessage = ref('')
const exportResult = ref(null)

const exportFormats = [
  { label: 'CSV', value: 'csv' },
  { label: 'JSON', value: 'json' },
  { label: 'HTML', value: 'html' }
]

const startExport = async () => {
  if (!mergePath.value) {
    alert('请输入数据库路径')
    return
  }

  loading.value = true
  exporting.value = true
  progress.value = 0
  progressMessage.value = '准备导出...'
  exportResult.value = null

  try {
    const requestData = {
      merge_path: mergePath.value,
      wxid: wxid.value || null,
      start_time: startTime.value ? new Date(startTime.value).getTime() / 1000 : null,
      end_time: endTime.value ? new Date(endTime.value).getTime() / 1000 : null,
      output_path: outputPath.value || null
    }

    progressMessage.value = '正在导出...'
    progress.value = 50

    let response
    if (selectedFormat.value === 'csv') {
      response = await fetch('/api/export/csv', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(requestData)
      })
    } else if (selectedFormat.value === 'json') {
      response = await fetch('/api/export/json', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(requestData)
      })
    } else {
      response = await fetch('/api/export/html', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(requestData)
      })
    }

    progress.value = 100
    progressMessage.value = '导出完成'

    const result = await response.json()
    exportResult.value = result.data || result

    if (exportResult.value.success) {
      progressMessage.value = '导出成功'
    } else {
      progressMessage.value = '导出失败'
    }
  } catch (error) {
    console.error('导出失败:', error)
    exportResult.value = {
      success: false,
      message: error.message || '导出失败'
    }
    progressMessage.value = '导出失败'
  } finally {
    loading.value = false
  }
}

// 从localStorage加载配置
const loadConfig = () => {
  mergePath.value = localStorage.getItem('merge_path') || ''
}

loadConfig()
</script>

<style scoped>
.export-view {
  padding: 24px;
}

.export-container {
  max-width: 800px;
}

.export-config {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin-bottom: 24px;
}

.config-section {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-weight: 500;
  margin-bottom: 8px;
  color: #333;
}

.config-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.time-range {
  display: flex;
  align-items: center;
  gap: 12px;
}

.time-separator {
  color: #666;
}

.format-options {
  display: flex;
  gap: 20px;
}

.format-option {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.export-button {
  width: 100%;
  padding: 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  margin-top: 20px;
}

.export-button:hover:not(:disabled) {
  background: #1976d2;
}

.export-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.export-progress {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin-bottom: 24px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.progress-status {
  color: #666;
  font-size: 14px;
}

.progress-bar-container {
  width: 100%;
  height: 8px;
  background: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: #2196f3;
  transition: width 0.3s;
}

.export-result {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.result-message {
  padding: 12px;
  border-radius: 4px;
  margin-bottom: 12px;
}

.result-message.success {
  background: #e8f5e9;
  color: #2e7d32;
}

.result-message.error {
  background: #ffebee;
  color: #c62828;
}

.result-file {
  font-size: 14px;
  color: #666;
}

.file-link {
  color: #2196f3;
  text-decoration: none;
  word-break: break-all;
}

.file-link:hover {
  text-decoration: underline;
}
</style>

