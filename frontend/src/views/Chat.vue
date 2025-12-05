<template>
  <div class="chat-view">
    <div class="chat-container">
      <!-- 联系人列表 -->
      <div class="contacts-sidebar">
        <div class="contacts-header">
          <h2>联系人</h2>
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="搜索联系人..."
            class="search-input"
          />
        </div>
        <div class="contacts-list">
          <div
            v-for="contact in filteredContacts"
            :key="contact.wxid"
            class="contact-item"
            :class="{ active: selectedContact?.wxid === contact.wxid }"
            @click="selectContact(contact)"
          >
            <div class="contact-avatar">{{ contact.wxid[0] }}</div>
            <div class="contact-info">
              <div class="contact-name">{{ contact.wxid }}</div>
              <div class="contact-msg-count">{{ contact.msg_count }} 条消息</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 聊天记录区域 -->
      <div class="chat-main">
        <div v-if="selectedContact" class="chat-header">
          <h3>{{ selectedContact.wxid }}</h3>
          <div class="msg-count">共 {{ selectedContact.msg_count }} 条消息</div>
        </div>
        <div v-else class="chat-header">
          <h3>请选择联系人</h3>
        </div>

        <div class="messages-container" ref="messagesContainer">
          <div
            v-for="message in messages"
            :key="message.id"
            class="message-item"
            :class="{ 'message-sent': message.is_sender === 1 }"
          >
            <div class="message-header">
              <span class="message-sender">
                {{ message.is_sender === 1 ? '我' : message.talker }}
              </span>
              <span class="message-time">{{ message.create_time_str }}</span>
            </div>
            <div class="message-content">
              <div class="message-type">{{ message.type_name }}</div>
              <div class="message-text">{{ message.content }}</div>
              <div v-if="message.src" class="message-src">
                <a :href="message.src" target="_blank">{{ message.src }}</a>
              </div>
            </div>
          </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>
        <div v-if="hasMore && !loading" class="load-more" @click="loadMore">
          加载更多
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { chatApi } from '../api/index.js'

const searchKeyword = ref('')
const contacts = ref([])
const selectedContact = ref(null)
const messages = ref([])
const loading = ref(false)
const hasMore = ref(true)
const currentPage = ref(0)
const pageSize = 50
const mergePath = ref('')

const filteredContacts = computed(() => {
  if (!searchKeyword.value) {
    return contacts.value
  }
  return contacts.value.filter(contact =>
    contact.wxid.toLowerCase().includes(searchKeyword.value.toLowerCase())
  )
})

const selectContact = (contact) => {
  selectedContact.value = contact
  messages.value = []
  currentPage.value = 0
  hasMore.value = true
  loadMessages()
}

const loadContacts = async () => {
  try {
    const response = await chatApi.getContacts({
      merge_path: mergePath.value
    })
    contacts.value = response.data.contacts
  } catch (error) {
    console.error('加载联系人失败:', error)
  }
}

const loadMessages = async () => {
  if (!selectedContact.value || loading.value) return

  loading.value = true
  try {
    const response = await chatApi.getMsgList({
      merge_path: mergePath.value,
      wxid: selectedContact.value.wxid,
      start: currentPage.value * pageSize,
      limit: pageSize
    })

    if (response.data.messages.length === 0) {
      hasMore.value = false
    } else {
      messages.value = [...response.data.messages, ...messages.value]
      currentPage.value++
    }
  } catch (error) {
    console.error('加载消息失败:', error)
  } finally {
    loading.value = false
  }
}

const loadMore = () => {
  if (hasMore.value && !loading.value) {
    loadMessages()
  }
}

onMounted(() => {
  // 从localStorage或配置中获取merge_path
  mergePath.value = localStorage.getItem('merge_path') || ''
  if (mergePath.value) {
    loadContacts()
  }
})
</script>

<style scoped>
.chat-view {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.chat-container {
  display: flex;
  height: 100%;
}

.contacts-sidebar {
  width: 300px;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
}

.contacts-header {
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
}

.contacts-header h2 {
  margin: 0 0 12px 0;
  font-size: 18px;
}

.search-input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.contacts-list {
  flex: 1;
  overflow-y: auto;
}

.contact-item {
  padding: 12px 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  border-bottom: 1px solid #f0f0f0;
}

.contact-item:hover {
  background: #f5f5f5;
}

.contact-item.active {
  background: #e3f2fd;
}

.contact-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #2196f3;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 12px;
  font-weight: bold;
}

.contact-info {
  flex: 1;
}

.contact-name {
  font-weight: 500;
  margin-bottom: 4px;
}

.contact-msg-count {
  font-size: 12px;
  color: #666;
}

.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.chat-header {
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
}

.chat-header h3 {
  margin: 0 0 4px 0;
}

.msg-count {
  font-size: 12px;
  color: #666;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.message-item {
  margin-bottom: 16px;
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
}

.message-item.message-sent {
  background: #e3f2fd;
  margin-left: auto;
  max-width: 70%;
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

.message-content {
  line-height: 1.6;
}

.message-type {
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.loading,
.load-more {
  text-align: center;
  padding: 16px;
  color: #666;
}

.load-more {
  cursor: pointer;
}

.load-more:hover {
  color: #2196f3;
}
</style>
