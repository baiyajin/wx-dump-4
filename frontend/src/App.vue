<template>
  <div class="min-h-screen bg-gray-50">
    <nav class="bg-white shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-bold text-gray-900">wx-dump-4</h1>
          </div>
          <div class="flex items-center space-x-4">
            <router-link to="/" class="text-gray-600 hover:text-gray-900">首页</router-link>
            <router-link to="/chat" class="text-gray-600 hover:text-gray-900">聊天记录</router-link>
            <router-link to="/statistics" class="text-gray-600 hover:text-gray-900">统计分析</router-link>
            <router-link to="/export" class="text-gray-600 hover:text-gray-900">数据导出</router-link>
            <router-link to="/settings" class="text-gray-600 hover:text-gray-900">设置</router-link>
          </div>
        </div>
      </div>
    </nav>
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <router-view />
    </main>
  </div>
</template>

<script setup>
</script>

