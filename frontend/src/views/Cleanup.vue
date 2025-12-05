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
              <span class="stat-value">{{ totalMessages.toLocaleString() }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">数据库大小：</span>
              <span class="stat-value">{{ dbSize }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">表数量：</span>
              <span class="stat-value">{{ tableCount }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">索引数量：</span>
              <span class="stat-value">{{ indexCount }}</span>
            </div>
          </div>
          
          <div v-if="tableInfos.length > 0" class="table-info">
            <h4>表信息</h4>
            <table class="info-table">
              <thead>
                <tr>
                  <th>表名</th>
                  <th>记录数</th>
                  <th>估算大小 (KB)</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="table in tableInfos" :key="table.name">
                  <td>{{ table.name }}</td>
                  <td>{{ table.row_count.toLocaleString() }}</td>
                  <td>{{ table.size_kb.toFixed(2) }}</td>
                </tr>
              </tbody>
            </table>
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
            <div v-if="cleanupOptions.deleteOldMessages" class="option-detail">
              <label>
                保留最近 <input type="number" v-model.number="cleanupOptions.daysToKeep" min="1" max="3650" class="days-input" /> 天的消息
              </label>
            </div>
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
            :disabled="analyzing || cleaning || vacuuming"
            class="action-button"
          >
            {{ analyzing ? '分析中...' : '分析存储' }}
          </button>
          <button
            @click="startCleanup"
            :disabled="analyzing || cleaning || vacuuming"
            class="action-button danger"
          >
            {{ cleaning ? '清理中...' : '开始清理' }}
          </button>
          <button
            @click="vacuumDatabase"
            :disabled="analyzing || cleaning || vacuuming"
            class="action-button"
          >
            {{ vacuuming ? '优化中...' : '优化数据库' }}
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
import { cleanupApi } from '../api/index.js'

const mergePath = ref('')
const totalMessages = ref(0)
const dbSize = ref('0 MB')
const dbSizeMB = ref(0)
const tableCount = ref(0)
const indexCount = ref(0)
const tableInfos = ref([])
const analyzing = ref(false)
const cleaning = ref(false)
const vacuuming = ref(false)
const cleanupResult = ref('')
const cleanupOptions = ref({
  deleteOldMessages: false,
  daysToKeep: 365,
  optimizeDatabase: false
})

const analyzeStorage = async () => {
  if (!mergePath.value) {
    cleanupResult.value = '请先在设置中配置数据库路径'
    return
  }

  analyzing.value = true
  cleanupResult.value = ''

  try {
    const response = await cleanupApi.analyzeStorage({
      merge_path: mergePath.value
    })
    
    const data = response.data
    totalMessages.value = data.total_messages
    dbSizeMB.value = data.db_size_mb
    dbSize.value = `${data.db_size_mb.toFixed(2)} MB`
    tableCount.value = data.table_count
    indexCount.value = data.index_count
    tableInfos.value = data.tables
    
    cleanupResult.value = `分析完成：共 ${data.total_messages} 条消息，数据库大小 ${data.db_size_mb.toFixed(2)} MB`
  } catch (error) {
    cleanupResult.value = '分析失败: ' + error.message
    console.error('Error analyzing storage:', error)
  } finally {
    analyzing.value = false
  }
}

const startCleanup = async () => {
  if (!mergePath.value) {
    cleanupResult.value = '请先在设置中配置数据库路径'
    return
  }

  if (!confirm('确定要执行清理操作吗？此操作不可恢复！')) {
    return
  }

  cleaning.value = true
  cleanupResult.value = ''

  try {
    const response = await cleanupApi.optimizeDatabase({
      merge_path: mergePath.value,
      delete_old_messages: cleanupOptions.value.deleteOldMessages,
      days_to_keep: cleanupOptions.value.deleteOldMessages ? cleanupOptions.value.daysToKeep : null,
      optimize_database: cleanupOptions.value.optimizeDatabase
    })
    
    cleanupResult.value = response.data.message || '清理完成'
    
    // 重新分析存储
    if (response.data.success) {
      await analyzeStorage()
    }
  } catch (error) {
    cleanupResult.value = '清理失败: ' + error.message
    console.error('Error cleaning up:', error)
  } finally {
    cleaning.value = false
  }
}

const vacuumDatabase = async () => {
  if (!mergePath.value) {
    cleanupResult.value = '请先在设置中配置数据库路径'
    return
  }

  if (!confirm('确定要优化数据库吗？这将释放未使用的空间。')) {
    return
  }

  vacuuming.value = true
  cleanupResult.value = ''

  try {
    const response = await cleanupApi.vacuumDatabase({
      merge_path: mergePath.value
    })
    
    cleanupResult.value = response.data.message || '优化完成'
    
    // 重新分析存储
    if (response.data.success) {
      await analyzeStorage()
    }
  } catch (error) {
    cleanupResult.value = '优化失败: ' + error.message
    console.error('Error vacuuming database:', error)
  } finally {
    vacuuming.value = false
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
  if (mergePath.value) {
    analyzeStorage()
  }
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

.option-detail {
  margin-left: 24px;
  margin-top: 8px;
  margin-bottom: 8px;
}

.days-input {
  width: 80px;
  padding: 4px 8px;
  margin: 0 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.table-info {
  margin-top: 24px;
}

.table-info h4 {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #333;
}

.info-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 8px;
}

.info-table th,
.info-table td {
  padding: 8px 12px;
  text-align: left;
  border-bottom: 1px solid #e0e0e0;
}

.info-table th {
  background: #f5f5f5;
  font-weight: 600;
  color: #666;
}

.info-table tr:hover {
  background: #f9f9f9;
}
</style>

