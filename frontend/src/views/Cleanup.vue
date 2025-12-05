<template>
  <div class="cleanup-view">
    <h2 class="text-2xl font-bold mb-6">清理工具</h2>

    <div class="cleanup-container">
      <div v-if="!mergePath" class="empty-state">
        <p>请先在设置中配置数据库路径</p>
      </div>

      <div v-else>
        <!-- 存储分析 -->
        <div class="cleanup-section">
          <h3 class="section-title">存储空间分析</h3>
          <div class="storage-stats">
            <div class="stat-item">
              <span class="stat-label">总消息数：</span>
              <span class="stat-value">{{ totalMessages }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">数据库大小：</span>
              <span class="stat-value">{{ dbSize }}</span>
            </div>
          </div>
        </div>

        <!-- 清理选项 -->
        <div class="cleanup-section">
          <h3 class="section-title">清理选项</h3>
          <div class="cleanup-options">
            <label class="option-item">
              <input type="checkbox" v-model="cleanupOptions.deleteOldMessages" />
              <span>删除旧消息（超过指定天数）</span>
            </label>
            <label class="option-item">
              <input type="checkbox" v-model="cleanupOptions.deleteMediaFiles" />
              <span>删除媒体文件缓存</span>
            </label>
            <label class="option-item">
              <input type="checkbox" v-model="cleanupOptions.optimizeDatabase" />
              <span>优化数据库（VACUUM）</span>
            </label>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="cleanup-actions">
          <button
            @click="analyzeStorage"
            :disabled="analyzing"
            class="action-button"
          >
            {{ analyzing ? '分析中...' : '分析存储' }}
          </button>
          <button
            @click="startCleanup"
            :disabled="cleaning"
            class="action-button danger"
          >
            {{ cleaning ? '清理中...' : '开始清理' }}
          </button>
        </div>

        <div v-if="cleanupResult" class="cleanup-result">
          {{ cleanupResult }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const mergePath = ref('')
const totalMessages = ref(0)
const dbSize = ref('0 MB')
const analyzing = ref(false)
const cleaning = ref(false)
const cleanupResult = ref('')
const cleanupOptions = ref({
  deleteOldMessages: false,
  deleteMediaFiles: false,
  optimizeDatabase: false
})

const analyzeStorage = async () => {
  analyzing.value = true
  cleanupResult.value = ''

  try {
    // TODO: 实现存储分析
    cleanupResult.value = '分析完成'
  } catch (error) {
    cleanupResult.value = '分析失败: ' + error.message
  } finally {
    analyzing.value = false
  }
}

const startCleanup = async () => {
  if (!confirm('确定要执行清理操作吗？此操作不可恢复！')) {
    return
  }

  cleaning.value = true
  cleanupResult.value = ''

  try {
    // TODO: 实现清理功能
    cleanupResult.value = '清理完成'
  } catch (error) {
    cleanupResult.value = '清理失败: ' + error.message
  } finally {
    cleaning.value = false
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
})
</script>

<style scoped>
.cleanup-view {
  padding: 24px;
}

.cleanup-container {
  max-width: 800px;
}

.empty-state {
  text-align: center;
  padding: 48px;
  color: #666;
}

.cleanup-section {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin-bottom: 24px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #333;
}

.storage-stats {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.stat-label {
  color: #666;
}

.stat-value {
  font-weight: 600;
  color: #2196f3;
}

.cleanup-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.option-item {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.cleanup-actions {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.action-button {
  flex: 1;
  padding: 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
}

.action-button:hover:not(:disabled) {
  background: #1976d2;
}

.action-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.action-button.danger {
  background: #f44336;
}

.action-button.danger:hover:not(:disabled) {
  background: #d32f2f;
}

.cleanup-result {
  padding: 12px;
  background: #f5f5f5;
  border-radius: 4px;
  text-align: center;
}
</style>

