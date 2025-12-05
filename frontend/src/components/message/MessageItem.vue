<template>
  <div class="message-item" :class="{ 'message-sent': isSender }">
    <div class="message-header">
      <span class="message-sender">{{ senderName }}</span>
      <span class="message-time">{{ createTimeStr }}</span>
    </div>
    <div class="message-body">
      <component 
        :is="messageComponent" 
        v-bind="messageProps"
      />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import MessageText from './MessageText.vue'
import MessageImg from './MessageImg.vue'
import MessageVideo from './MessageVideo.vue'
import MessageAudio from './MessageAudio.vue'
import MessageFile from './MessageFile.vue'
import MessageEmoji from './MessageEmoji.vue'
import MessageOther from './MessageOther.vue'

const props = defineProps({
  msgType: {
    type: Number,
    required: true
  },
  subType: {
    type: Number,
    required: true
  },
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
  },
  extra: {
    type: Object,
    default: () => ({})
  },
  isSender: {
    type: Number,
    default: 0
  },
  senderName: {
    type: String,
    default: ''
  },
  createTimeStr: {
    type: String,
    required: true
  }
})

const messageComponent = computed(() => {
  const type = props.msgType
  const subType = props.subType

  if (type === 1 && subType === 0) {
    return MessageText
  } else if (type === 3 && subType === 0) {
    return MessageImg
  } else if (type === 43 && subType === 0) {
    return MessageVideo
  } else if (type === 34 && subType === 0) {
    return MessageAudio
  } else if ((type === 49 && subType === 0) || (type === 49 && subType === 6)) {
    return MessageFile
  } else if (type === 47 && subType === 0) {
    return MessageEmoji
  } else {
    return MessageOther
  }
})

const messageProps = computed(() => {
  return {
    content: props.content,
    src: props.src,
    extra: props.extra,
    typeName: props.typeName
  }
})
</script>

<style scoped>
.message-item {
  margin-bottom: 16px;
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
  max-width: 70%;
}

.message-item.message-sent {
  background: #e3f2fd;
  margin-left: auto;
}

.message-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 12px;
}

.message-sender {
  font-weight: bold;
  color: #2196f3;
}

.message-time {
  color: #999;
}

.message-body {
  line-height: 1.6;
}
</style>

