<template>
  <div class="message-video">
    <div v-if="src" class="video-container">
      <video 
        :src="getVideoUrl(src)" 
        controls
        class="message-video-player"
        preload="metadata"
      >
        您的浏览器不支持视频播放
      </video>
    </div>
    <div v-else class="video-placeholder">
      <span>视频</span>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  content: {
    type: String,
    default: '视频'
  },
  src: {
    type: String,
    default: ''
  }
})

const getVideoUrl = (path) => {
  if (!path) return ''
  if (path.startsWith('http://') || path.startsWith('https://') || path.startsWith('file://')) {
    return path
  }
  return `/api/media/video?path=${encodeURIComponent(path)}`
}
</script>

<style scoped>
.message-video {
  max-width: 400px;
}

.video-container {
  border-radius: 8px;
  overflow: hidden;
}

.message-video-player {
  max-width: 100%;
  height: auto;
  display: block;
}

.video-placeholder {
  padding: 20px;
  text-align: center;
  background: #f5f5f5;
  border-radius: 8px;
  color: #666;
}
</style>

