<template>
  <div class="settings-view">
    <h2 class="text-2xl font-bold mb-6">设置</h2>

    <div class="settings-container">
      <!-- 数据库配置 -->
      <div class="settings-section">
        <h3 class="section-title">数据库配置</h3>
        <div class="setting-item">
          <label class="setting-label">合并数据库路径</label>
          <input
            v-model="mergePath"
            type="text"
            placeholder="请输入合并后的数据库路径"
            class="setting-input"
          />
          <button @click="saveMergePath" class="save-button">保存</button>
        </div>
      </div>

      <!-- 导出配置 -->
      <div class="settings-section">
        <h3 class="section-title">导出配置</h3>
        <div class="setting-item">
          <label class="setting-label">默认导出路径</label>
          <input
            v-model="exportPath"
            type="text"
            placeholder="请输入默认导出路径"
            class="setting-input"
          />
          <button @click="saveExportPath" class="save-button">保存</button>
        </div>
      </div>

      <!-- 显示配置 -->
      <div class="settings-section">
        <h3 class="section-title">显示配置</h3>
        <div class="setting-item">
          <label class="setting-label">每页消息数量</label>
          <input
            v-model.number="pageSize"
            type="number"
            min="10"
            max="1000"
            class="setting-input"
          />
          <button @click="savePageSize" class="save-button">保存</button>
        </div>
        <div class="setting-item">
          <label class="setting-label">主题</label>
          <select v-model="theme" @change="saveTheme" class="setting-input">
            <option value="light">浅色</option>
            <option value="dark">深色</option>
            <option value="auto">跟随系统</option>
          </select>
        </div>
        <div class="setting-item">
          <label class="setting-label">语言</label>
          <select v-model="language" @change="saveLanguage" class="setting-input">
            <option value="zh-CN">简体中文</option>
            <option value="en-US">English</option>
          </select>
        </div>
      </div>

      <!-- 操作 -->
      <div class="settings-section">
        <h3 class="section-title">操作</h3>
        <div class="setting-item">
          <button @click="clearCache" class="action-button">清除缓存</button>
          <button @click="resetSettings" class="action-button danger">重置设置</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const mergePath = ref('')
const exportPath = ref('')
const pageSize = ref(50)
const theme = ref('light')
const language = ref('zh-CN')

const saveMergePath = () => {
  localStorage.setItem('merge_path', mergePath.value)
  alert('保存成功')
}

const saveExportPath = () => {
  localStorage.setItem('export_path', exportPath.value)
  alert('保存成功')
}

const savePageSize = () => {
  localStorage.setItem('page_size', pageSize.value.toString())
  alert('保存成功')
}

const clearCache = () => {
  if (confirm('确定要清除缓存吗？')) {
    localStorage.removeItem('user_list')
    localStorage.removeItem('contacts_cache')
    alert('缓存已清除')
  }
}

const saveTheme = () => {
  localStorage.setItem('darkMode', theme.value === 'dark' ? 'true' : 'false')
  localStorage.setItem('theme', theme.value)
  
  // 触发主题更新
  const event = new CustomEvent('theme-change', { detail: { theme: theme.value } })
  window.dispatchEvent(event)
  
  alert('主题设置已保存')
}

const saveLanguage = () => {
  localStorage.setItem('language', language.value)
  alert('语言设置已保存，刷新页面后生效')
}

const resetSettings = () => {
  if (confirm('确定要重置所有设置吗？')) {
    localStorage.clear()
    mergePath.value = ''
    exportPath.value = ''
    pageSize.value = 50
    theme.value = 'light'
    language.value = 'zh-CN'
    
    // 触发主题更新
    const event = new CustomEvent('theme-change', { detail: { theme: 'light' } })
    window.dispatchEvent(event)
    
    alert('设置已重置')
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
  exportPath.value = localStorage.getItem('export_path') || ''
  const savedPageSize = localStorage.getItem('page_size')
  if (savedPageSize) {
    pageSize.value = parseInt(savedPageSize, 10)
  }
  
  // 加载主题设置
  const savedTheme = localStorage.getItem('theme') || 'light'
  const savedDarkMode = localStorage.getItem('darkMode')
  if (savedDarkMode === 'true') {
    theme.value = 'dark'
  } else if (savedTheme) {
    theme.value = savedTheme
  }
  
  // 加载语言设置
  language.value = localStorage.getItem('language') || 'zh-CN'
})
</script>

<style scoped>
.settings-view {
  padding: 24px;
}

.settings-container {
  max-width: 800px;
}

.settings-section {
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

.setting-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.setting-label {
  min-width: 150px;
  font-weight: 500;
  color: #333;
}

.setting-input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.save-button {
  padding: 10px 20px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.save-button:hover {
  background: #1976d2;
}

.action-button {
  padding: 10px 20px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  margin-right: 12px;
}

.action-button:hover {
  background: #1976d2;
}

.action-button.danger {
  background: #f44336;
}

.action-button.danger:hover {
  background: #d32f2f;
}
</style>
