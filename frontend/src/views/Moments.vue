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
            <div v-if="moment.content" class="moment-content">{{ moment.content }}</div>
            <div v-if="moment.media_list && moment.media_list.length > 0" class="moment-media">
              <div class="media-grid" :class="`grid-${Math.min(moment.media_list.length, 9)}`">
                <img
                  v-for="(media, index) in moment.media_list"
                  :key="index"
                  :src="media"
                  alt="朋友圈图片"
                  @click="viewImage(media, moment.media_list)"
                  class="media-item"
                />
              </div>
            </div>
          </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>
        <div v-if="error" class="error">{{ error }}</div>
      </div>
    </div>

    <!-- 图片查看器 -->
    <div v-if="imageViewer.show" class="image-viewer" @click="closeImageViewer">
      <img :src="imageViewer.current" alt="朋友圈图片" class="viewer-image" />
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
const imageViewer = ref({ show: false, current: '', list: [] })

const viewImage = (src, list) => {
  imageViewer.value = {
    show: true,
    current: src,
    list: list || [src]
  }
}

const closeImageViewer = () => {
  imageViewer.value = { show: false, current: '', list: [] }
}

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
  margin-bottom: 12px;
}

.moment-media {
  margin-top: 12px;
}

.media-grid {
  display: grid;
  gap: 8px;
}

.media-grid.grid-1 {
  grid-template-columns: 1fr;
}

.media-grid.grid-2,
.media-grid.grid-4 {
  grid-template-columns: repeat(2, 1fr);
}

.media-grid.grid-3,
.media-grid.grid-5,
.media-grid.grid-6,
.media-grid.grid-7,
.media-grid.grid-8,
.media-grid.grid-9 {
  grid-template-columns: repeat(3, 1fr);
}

.media-item {
  width: 100%;
  height: 200px;
  object-fit: cover;
  border-radius: 4px;
  cursor: pointer;
  transition: transform 0.2s;
}

.media-item:hover {
  transform: scale(1.05);
}

.image-viewer {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  cursor: pointer;
}

.viewer-image {
  max-width: 90%;
  max-height: 90%;
  object-fit: contain;
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

