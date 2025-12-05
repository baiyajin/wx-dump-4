<template>
  <div class="message-img">
    <div v-if="src" class="img-container">
      <img 
        :src="getImageUrl(src)" 
        :alt="content"
        @error="handleError"
        @click="previewImage"
        class="message-image"
      />
    </div>
    <div v-else class="img-placeholder">
      <span>图片</span>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  content: {
    type: String,
    default: '图片'
  },
  src: {
    type: String,
    default: ''
  }
})

const imageError = ref(false)

const getImageUrl = (path) => {
  if (!path) return ''
  // 如果是完整路径，直接返回
  if (path.startsWith('http://') || path.startsWith('https://') || path.startsWith('file://')) {
    return path
  }
  // 否则尝试从微信文件路径读取
  return `/api/media/img?path=${encodeURIComponent(path)}`
}

const handleError = () => {
  imageError.value = true
}

const previewImage = () => {
  if (props.src) {
    window.open(getImageUrl(props.src), '_blank')
  }
}
</script>

<style scoped>
.message-img {
  max-width: 300px;
}

.img-container {
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
}

.message-image {
  max-width: 100%;
  height: auto;
  display: block;
}

.img-placeholder {
  padding: 20px;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #666;
}
</style>

