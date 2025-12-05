<template>
  <div class="message-other">
    <div class="other-container">
      <div class="other-type">{{ typeName }}</div>
      <div class="other-content">{{ content }}</div>
      <div v-if="src" class="other-src">
        <a :href="src" target="_blank" v-if="isUrl(src)">{{ src }}</a>
        <span v-else>{{ src }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  typeName: {
    type: String,
    required: true
  },
  content: {
    type: String,
    default: ''
  },
  src: {
    type: String,
    default: ''
  }
})

const isUrl = (str) => {
  if (!str) return false
  return str.startsWith('http://') || str.startsWith('https://')
}
</script>

<style scoped>
.message-other {
  display: inline-block;
  max-width: 400px;
}

.other-container {
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
}

.other-type {
  font-size: 12px;
  color: #666;
  margin-bottom: 8px;
}

.other-content {
  word-wrap: break-word;
  white-space: pre-wrap;
  line-height: 1.6;
}

.other-src {
  margin-top: 8px;
  font-size: 12px;
}

.other-src a {
  color: #2196f3;
  text-decoration: none;
}

.other-src a:hover {
  text-decoration: underline;
}
</style>

