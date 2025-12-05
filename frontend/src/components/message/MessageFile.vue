<template>
  <div class="message-file">
    <div class="file-container">
      <div class="file-icon">📄</div>
      <div class="file-info">
        <div class="file-name">{{ fileName }}</div>
        <div v-if="fileSize" class="file-size">{{ formatFileSize(fileSize) }}</div>
      </div>
      <a 
        v-if="src" 
        :href="getFileUrl(src)" 
        :download="fileName"
        class="download-button"
      >
        下载
      </a>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  content: {
    type: String,
    default: '文件'
  },
  src: {
    type: String,
    default: ''
  },
  extra: {
    type: Object,
    default: () => ({})
  }
})

const fileName = computed(() => {
  if (props.content && props.content !== '文件') {
    return props.content
  }
  if (props.src) {
    const parts = props.src.split(/[/\\]/)
    return parts[parts.length - 1] || '未知文件'
  }
  return '未知文件'
})

const fileSize = computed(() => {
  return props.extra.file_size || null
})

const getFileUrl = (path) => {
  if (!path) return ''
  if (path.startsWith('http://') || path.startsWith('https://')) {
    return path
  }
  return `/api/media/file?path=${encodeURIComponent(path)}`
}

const formatFileSize = (bytes) => {
  if (!bytes) return ''
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
  return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB'
}
</script>

<style scoped>
.message-file {
  display: inline-block;
  min-width: 250px;
}

.file-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
}

.file-icon {
  font-size: 32px;
}

.file-info {
  flex: 1;
}

.file-name {
  font-weight: 500;
  margin-bottom: 4px;
  word-break: break-all;
}

.file-size {
  font-size: 12px;
  color: #666;
}

.download-button {
  padding: 6px 12px;
  background: #2196f3;
  color: white;
  text-decoration: none;
  border-radius: 4px;
  font-size: 14px;
}

.download-button:hover {
  background: #1976d2;
}
</style>

