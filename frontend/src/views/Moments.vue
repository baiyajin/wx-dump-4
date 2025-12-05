<template>
  <div class="moments-view">
    <h2 class="text-2xl font-bold mb-6">朋友圈</h2>

    <div class="moments-container">
      <div v-if="!mergePath" class="empty-state">
        <p>请先在设置中配置数据库路径</p>
      </div>

      <div v-else>
        <div class="moments-list">
          <div
            v-for="moment in moments"
            :key="moment.id"
            class="moment-item"
          >
            <div class="moment-header">
              <div class="moment-author">{{ moment.user_name }}</div>
              <div class="moment-time">{{ moment.create_time_str }}</div>
            </div>
            <div class="moment-content">{{ moment.content }}</div>
          </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>
        <div v-if="error" class="error">{{ error }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { momentsApi } from '../api/index.js'

const moments = ref([])
const loading = ref(false)
const error = ref(null)
const mergePath = ref('')
const total = ref(0)
const currentPage = ref(0)
const pageSize = 50

const loadMoments = async () => {
  if (!mergePath.value) {
    error.value = '请先在设置中配置数据库路径'
    return
  }

  loading.value = true
  error.value = null

  try {
    const response = await momentsApi.getMomentsList({
      merge_path: mergePath.value,
      start: currentPage.value * pageSize,
      limit: pageSize
    })
    
    moments.value = response.data.moments
    total.value = response.data.total
  } catch (err) {
    error.value = '加载朋友圈失败: ' + err.message
    console.error('Error loading moments:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
  if (mergePath.value) {
    loadMoments()
  }
})
</script>

<style scoped>
.moments-view {
  padding: 24px;
}

.moments-container {
  max-width: 800px;
  margin: 0 auto;
}

.empty-state {
  text-align: center;
  padding: 48px;
  color: #666;
}

.moments-list {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.moment-item {
  background: white;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.moment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.moment-author {
  font-weight: 600;
  font-size: 16px;
}

.moment-time {
  font-size: 12px;
  color: #999;
}

.moment-content {
  line-height: 1.6;
  word-wrap: break-word;
}

.loading,
.error {
  text-align: center;
  padding: 24px;
  color: #666;
}

.error {
  color: #f44336;
}
</style>

