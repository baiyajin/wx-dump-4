<template>
  <div>
    <h2 class="text-2xl font-bold mb-6">微信信息</h2>
    
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <button 
        @click="fetchWxInfo" 
        :disabled="loading"
        class="btn"
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
        class="bg-white rounded-lg shadow p-6"
      >
        <h3 class="text-lg font-semibold mb-4">进程 ID: {{ info.pid }}</h3>
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
</template>

<script setup>
import { ref } from 'vue'
import { wxApi } from '../api'

const loading = ref(false)
const error = ref(null)
const wxInfo = ref([])

const fetchWxInfo = async () => {
  loading.value = true
  error.value = null
  
  try {
    const response = await wxApi.getWxInfo()
    wxInfo.value = response.data
    if (wxInfo.value.length === 0) {
      error.value = '未找到运行中的微信进程'
    }
  } catch (err) {
    error.value = err.response?.data?.error || err.message || '获取微信信息失败'
    console.error('Error fetching wx info:', err)
  } finally {
    loading.value = false
  }
}
</script>

