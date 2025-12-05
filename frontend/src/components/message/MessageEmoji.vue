<template>
  <div class="message-emoji">
    <div v-if="src" class="emoji-container">
      <img 
        :src="getEmojiUrl(src)" 
        :alt="content"
        @error="handleError"
        class="emoji-image"
      />
    </div>
    <div v-else class="emoji-placeholder">
      <span>表情</span>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  content: {
    type: String,
    default: '表情'
  },
  src: {
    type: String,
    default: ''
  }
})

const imageError = ref(false)

const getEmojiUrl = (path) => {
  if (!path) return ''
  if (path.startsWith('http://') || path.startsWith('https://') || path.startsWith('file://')) {
    return path
  }
  return `/api/media/emoji?path=${encodeURIComponent(path)}`
}

const handleError = () => {
  imageError.value = true
}
</script>

<style scoped>
.message-emoji {
  display: inline-block;
}

.emoji-container {
  display: inline-block;
}

.emoji-image {
  max-width: 200px;
  max-height: 200px;
  display: block;
}

.emoji-placeholder {
  padding: 20px;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #666;
  display: inline-block;
}
</style>

