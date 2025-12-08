<template>
  <div 
    class="message-item" 
    :class="{ 'message-sent': isSender }"
    @contextmenu.prevent="showContextMenu"
    @mouseenter="showActions = true"
    @mouseleave="showActions = false"
  >
    <div class="message-header">
      <span class="message-sender">{{ senderName }}</span>
      <span class="message-time">{{ createTimeStr }}</span>
      <div v-if="showActions" class="message-actions">
        <button @click="copyMessage" class="action-btn" title="复制">📋</button>
      </div>
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
import { computed, ref } from 'vue'
import MessageText from './MessageText.vue'
import MessageImg from './MessageImg.vue'
import MessageVideo from './MessageVideo.vue'
import MessageAudio from './MessageAudio.vue'
import MessageFile from './MessageFile.vue'
import MessageEmoji from './MessageEmoji.vue'
import MessageOther from './MessageOther.vue'

const showActions = ref(false)

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

const copyMessage = async () => {
  let textToCopy = props.content
  
  // 如果是其他类型消息，尝试从extra中提取信息
  if (props.typeName !== '文本' && props.extra) {
    if (props.extra.title) {
      textToCopy = props.extra.title
    } else if (props.extra.url) {
      textToCopy = props.extra.url
    } else if (props.src) {
      textToCopy = props.src
    }
  }
  
  try {
    await navigator.clipboard.writeText(textToCopy)
    // 显示复制成功提示
    const btn = event?.target
    if (btn) {
      const originalText = btn.textContent
      btn.textContent = '✓'
      setTimeout(() => {
        btn.textContent = originalText
      }, 1000)
    }
  } catch (err) {
    // 降级方案：使用传统方法
    const textArea = document.createElement('textarea')
    textArea.value = textToCopy
    textArea.style.position = 'fixed'
    textArea.style.opacity = '0'
    document.body.appendChild(textArea)
    textArea.select()
    try {
      document.execCommand('copy')
      alert('已复制到剪贴板')
    } catch (e) {
      alert('复制失败，请手动复制')
    }
    document.body.removeChild(textArea)
  }
}

const showContextMenu = (event) => {
  copyMessage()
}
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

.message-actions {
  display: flex;
  gap: 4px;
  margin-left: 8px;
}

.action-btn {
  background: rgba(0, 0, 0, 0.1);
  border: none;
  border-radius: 4px;
  padding: 4px 8px;
  cursor: pointer;
  font-size: 14px;
  transition: background 0.2s;
}

.action-btn:hover {
  background: rgba(0, 0, 0, 0.2);
}

.message-body {
  line-height: 1.6;
}
</style>

