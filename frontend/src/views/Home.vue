<template>
  <div>
    <h2 class="text-2xl font-bold mb-6">首页</h2>
    
    <!-- 快速操作 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div class="bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition" @click="$router.push('/chat')">
        <div class="text-2xl mb-2">💬</div>
        <div class="font-semibold">聊天记录</div>
        <div class="text-sm text-gray-500">查看和管理聊天记录</div>
      </div>
      <div class="bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition" @click="$router.push('/statistics')">
        <div class="text-2xl mb-2">📊</div>
        <div class="font-semibold">统计分析</div>
        <div class="text-sm text-gray-500">查看聊天统计数据</div>
      </div>
      <div class="bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition" @click="$router.push('/export')">
        <div class="text-2xl mb-2">📤</div>
        <div class="font-semibold">数据导出</div>
        <div class="text-sm text-gray-500">导出聊天记录</div>
      </div>
    </div>

    <!-- 微信信息 -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-semibold">微信信息</h3>
        <button 
          @click="fetchWxInfo" 
          :disabled="loading"
          class="btn btn-primary"
        >
          {{ loading ? '获取中...' : '获取微信信息' }}
        </button>
      </div>

      <div v-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
        {{ error }}
      </div>

      <div v-if="wxInfo.length > 0" class="space-y-4">
        <div 
          v-for="info in wxInfo" 
          :key="info.pid"
          class="border rounded-lg p-4"
        >
          <h4 class="text-md font-semibold mb-3">进程 ID: {{ info.pid }}</h4>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <span class="text-gray-600">版本:</span>
              <span class="ml-2 font-medium">{{ info.version }}</span>
            </div>
            <div>
              <span class="text-gray-600">账号:</span>
              <span class="ml-2 font-medium">{{ info.account || 'N/A' }}</span>
            </div>
            <div>
              <span class="text-gray-600">昵称:</span>
              <span class="ml-2 font-medium">{{ info.nickname || 'N/A' }}</span>
            </div>
            <div>
              <span class="text-gray-600">手机号:</span>
              <span class="ml-2 font-medium">{{ info.mobile || 'N/A' }}</span>
            </div>
            <div>
              <span class="text-gray-600">微信ID:</span>
              <span class="ml-2 font-medium">{{ info.wxid || 'N/A' }}</span>
            </div>
            <div>
              <span class="text-gray-600">密钥:</span>
              <span class="ml-2 font-mono text-sm">{{ info.key ? info.key.substring(0, 16) + '...' : 'N/A' }}</span>
            </div>
            <div class="col-span-2">
              <span class="text-gray-600">文件路径:</span>
              <span class="ml-2 text-sm">{{ info.wx_dir || 'N/A' }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 统计概览 -->
    <div v-if="wxInfo.length > 0 && currentWxInfo" class="bg-white rounded-lg shadow p-6">
      <h3 class="text-lg font-semibold mb-4">统计概览</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="text-center p-4 bg-blue-50 rounded">
          <div class="text-2xl font-bold text-blue-600">{{ stats.totalMessages || 0 }}</div>
          <div class="text-sm text-gray-600 mt-1">总消息数</div>
        </div>
        <div class="text-center p-4 bg-green-50 rounded">
          <div class="text-2xl font-bold text-green-600">{{ stats.totalContacts || 0 }}</div>
          <div class="text-sm text-gray-600 mt-1">联系人</div>
        </div>
        <div class="text-center p-4 bg-purple-50 rounded">
          <div class="text-2xl font-bold text-purple-600">{{ stats.totalFavorites || 0 }}</div>
          <div class="text-sm text-gray-600 mt-1">收藏</div>
        </div>
        <div class="text-center p-4 bg-orange-50 rounded">
          <div class="text-2xl font-bold text-orange-600">{{ stats.totalMoments || 0 }}</div>
          <div class="text-sm text-gray-600 mt-1">朋友圈</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { wxApi, chatApi, favoriteApi, momentsApi } from '../api'

const loading = ref(false)
const error = ref(null)
const wxInfo = ref([])
const stats = ref({
  totalMessages: 0,
  totalContacts: 0,
  totalFavorites: 0,
  totalMoments: 0,
})

const currentWxInfo = computed(() => {
  return wxInfo.value.length > 0 ? wxInfo.value[0] : null
})

const fetchWxInfo = async () => {
  loading.value = true
  error.value = null
  
  try {
    const response = await wxApi.getWxInfo()
    wxInfo.value = response.data
    if (wxInfo.value.length === 0) {
      error.value = '未找到运行中的微信进程'
    } else {
      // 获取统计信息
      await fetchStats()
    }
  } catch (err) {
    error.value = err.response?.data?.error || err.message || '获取微信信息失败'
    console.error('Error fetching wx info:', err)
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  if (!currentWxInfo.value?.wx_dir) return
  
  try {
    // 这里需要根据实际的数据库路径获取统计信息
    // 暂时使用占位数据
    stats.value = {
      totalMessages: 0,
      totalContacts: 0,
      totalFavorites: 0,
      totalMoments: 0,
    }
  } catch (err) {
    console.error('Error fetching stats:', err)
  }
}

onMounted(() => {
  // 自动获取微信信息
  fetchWxInfo()
})
</script>

