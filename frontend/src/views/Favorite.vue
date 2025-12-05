<template>
  <div class="favorite-view">
    <h2 class="text-2xl font-bold mb-6">收藏</h2>

    <div class="favorite-container">
      <div v-if="!mergePath" class="empty-state">
        <p>请先在设置中配置数据库路径</p>
      </div>

      <div v-else>
        <div class="favorite-header">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索收藏内容..."
            class="search-input"
          />
        </div>

        <div class="favorites-list">
          <div
            v-for="favorite in filteredFavorites"
            :key="favorite.id"
            class="favorite-item"
          >
            <div class="favorite-type">{{ favorite.type_name }}</div>
            <div class="favorite-content">{{ favorite.content }}</div>
            <div class="favorite-time">{{ favorite.create_time_str }}</div>
          </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>
        <div v-if="error" class="error">{{ error }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { favoriteApi } from '../api/index.js'

const searchKeyword = ref('')
const favorites = ref([])
const loading = ref(false)
const error = ref(null)
const mergePath = ref('')
const total = ref(0)
const currentPage = ref(0)
const pageSize = 50

const filteredFavorites = computed(() => {
  if (!searchKeyword.value) {
    return favorites.value
  }
  const keyword = searchKeyword.value.toLowerCase()
  return favorites.value.filter(fav =>
    fav.content.toLowerCase().includes(keyword) ||
    fav.type_name.toLowerCase().includes(keyword)
  )
})

const loadFavorites = async () => {
  if (!mergePath.value) {
    error.value = '请先在设置中配置数据库路径'
    return
  }

  loading.value = true
  error.value = null

  try {
    const response = await favoriteApi.getFavoriteList({
      merge_path: mergePath.value,
      start: currentPage.value * pageSize,
      limit: pageSize
    })
    
    favorites.value = response.data.favorites
    total.value = response.data.total
  } catch (err) {
    error.value = '加载收藏失败: ' + err.message
    console.error('Error loading favorites:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
  if (mergePath.value) {
    loadFavorites()
  }
})
</script>

<style scoped>
.favorite-view {
  padding: 24px;
}

.favorite-container {
  max-width: 1200px;
}

.empty-state {
  text-align: center;
  padding: 48px;
  color: #666;
}

.favorite-header {
  margin-bottom: 24px;
}

.search-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.favorites-list {
  display: grid;
  gap: 16px;
}

.favorite-item {
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.favorite-type {
  font-size: 12px;
  color: #666;
  margin-bottom: 8px;
}

.favorite-content {
  line-height: 1.6;
  margin-bottom: 8px;
  word-wrap: break-word;
}

.favorite-time {
  font-size: 12px;
  color: #999;
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

