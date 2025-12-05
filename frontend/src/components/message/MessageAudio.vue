<template>
  <div class="message-audio">
    <div class="audio-container">
      <button @click="togglePlay" class="play-button">
        {{ isPlaying ? '⏸' : '▶' }}
      </button>
      <div class="audio-info">
        <div class="audio-label">语音消息</div>
        <div v-if="duration" class="audio-duration">{{ duration }}秒</div>
      </div>
      <audio 
        ref="audioRef"
        :src="getAudioUrl(src)"
        @ended="onEnded"
        @timeupdate="onTimeUpdate"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

const props = defineProps({
  content: {
    type: String,
    default: '语音'
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

const audioRef = ref(null)
const isPlaying = ref(false)
const currentTime = ref(0)

const duration = computed(() => {
  if (props.extra.voicelength) {
    return parseFloat(props.extra.voicelength).toFixed(1)
  }
  return null
})

const getAudioUrl = (path) => {
  if (!path) return ''
  if (path.startsWith('http://') || path.startsWith('https://') || path.startsWith('file://')) {
    return path
  }
  return `/api/media/audio?path=${encodeURIComponent(path)}`
}

const togglePlay = () => {
  if (!audioRef.value) return
  
  if (isPlaying.value) {
    audioRef.value.pause()
  } else {
    audioRef.value.play()
  }
  isPlaying.value = !isPlaying.value
}

const onEnded = () => {
  isPlaying.value = false
  currentTime.value = 0
}

const onTimeUpdate = () => {
  if (audioRef.value) {
    currentTime.value = audioRef.value.currentTime
  }
}
</script>

<style scoped>
.message-audio {
  display: inline-block;
  min-width: 200px;
}

.audio-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
}

.play-button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: none;
  background: #2196f3;
  color: white;
  font-size: 18px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.play-button:hover {
  background: #1976d2;
}

.audio-info {
  flex: 1;
}

.audio-label {
  font-weight: 500;
  margin-bottom: 4px;
}

.audio-duration {
  font-size: 12px;
  color: #666;
}
</style>

