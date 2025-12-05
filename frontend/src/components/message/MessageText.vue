<template>
  <div class="message-text">
    <div class="text-content">{{ content }}</div>
  </div>
</template>

<script setup>
defineProps({
  content: {
    type: String,
    required: true
  }
})
</script>

<style scoped>
.message-text {
  word-wrap: break-word;
  word-break: break-all;
}

.text-content {
  white-space: pre-wrap;
  line-height: 1.6;
}
</style>

