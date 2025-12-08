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
          <div class="header-info">
            <h3>{{ selectedContact.wxid }}</h3>
            <div class="msg-count">共 {{ selectedContact.msg_count }} 条消息</div>
          </div>
          <div class="header-actions">
            <div class="search-box">
              <input
                v-model="searchKeyword"
                type="text"
                placeholder="搜索消息..."
                class="search-input"
                @keyup.enter="performSearch"
                @input="onSearchInput"
              />
              <button v-if="searchKeyword" @click="clearSearch" class="clear-btn">×</button>
              <button @click="performSearch" class="search-btn">搜索</button>
            </div>
          </div>
        </div>
        <div v-else class="chat-header">
          <h3>请选择联系人</h3>
        </div>

        <!-- 搜索结果 -->
        <div v-if="searchMode && searchResults.length > 0" class="search-results">
          <div class="search-results-header">
            <span>找到 {{ searchResults.length }} 条结果（关键词：{{ searchKeyword }}）</span>
            <button @click="exitSearch" class="exit-search-btn">返回聊天</button>
          </div>
          <div class="search-results-list">
            <div
              v-for="message in searchResults"
              :key="message.id"
              class="search-result-item"
              @click="jumpToMessage(message)"
            >
              <div class="result-time">{{ message.create_time_str }}</div>
              <div class="result-content" v-html="highlightKeyword(message.content, searchKeyword)"></div>
            </div>
          </div>
        </div>

        <!-- 正常消息列表 -->
        <div 
          v-else
          class="messages-container" 
          ref="messagesContainer"
          @scroll="handleScroll"
        >
          <div v-if="loading && messages.length === 0" class="loading">加载中...</div>
          <div v-else-if="messages.length === 0" class="empty-state">
            <p>暂无消息</p>
          </div>
          <template v-else>
            <div v-if="loading" class="loading-top">加载历史消息...</div>
            <MessageItem
              v-for="message in messages"
              :key="message.id"
              :msg-type="message.msg_type"
              :sub-type="message.sub_type"
              :type-name="message.type_name"
              :content="highlightKeyword(message.content, currentSearchKeyword)"
              :src="message.src"
              :extra="message.extra"
              :is-sender="message.is_sender"
              :sender-name="message.is_sender === 1 ? '我' : (userList[message.talker]?.nickname || message.talker)"
              :create-time-str="message.create_time_str"
            />
            <div v-if="!hasMore && messages.length > 0" class="no-more">
              没有更多消息了
            </div>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { chatApi } from '../api/index.js'
import MessageItem from '../components/message/MessageItem.vue'

const searchKeyword = ref('')
const contacts = ref([])
const selectedContact = ref(null)
const messages = ref([])
const userList = ref({})
const loading = ref(false)
const hasMore = ref(true)
const currentPage = ref(0)
const pageSize = 50

const filteredContacts = computed(() => {
  let result = contacts.value
  
  // 搜索过滤
  if (searchKeyword.value) {
    result = result.filter(contact =>
      contact.wxid.toLowerCase().includes(searchKeyword.value.toLowerCase())
    )
  }
  
  // 按消息数量排序
  return result.sort((a, b) => (b.msg_count || 0) - (a.msg_count || 0))
})

const selectContact = (contact) => {
  selectedContact.value = contact
  messages.value = []
  currentPage.value = 0
  hasMore.value = true
  userList.value = {}
  loadMessages(true) // 首次加载滚动到底部
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

const loadMessages = async (scrollToBottom = false) => {
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
      const oldScrollHeight = messagesContainer.value?.scrollHeight || 0
      messages.value = [...response.data.messages, ...messages.value]
      if (response.data.user_list) {
        userList.value = { ...userList.value, ...response.data.user_list }
      }
      currentPage.value++
      
      // 保持滚动位置（无限滚动向上加载）
      if (messagesContainer.value && !scrollToBottom) {
        const newScrollHeight = messagesContainer.value.scrollHeight
        messagesContainer.value.scrollTop = newScrollHeight - oldScrollHeight
      } else if (scrollToBottom && messagesContainer.value) {
        // 首次加载或新消息时滚动到底部
        setTimeout(() => {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
        }, 100)
      }
    }
  } catch (error) {
    console.error('加载消息失败:', error)
  } finally {
    loading.value = false
  }
}

const handleScroll = () => {
  if (!messagesContainer.value || !hasMore.value || loading.value || searchMode.value) return
  
  // 当滚动到顶部附近时加载更多
  if (messagesContainer.value.scrollTop < 100) {
    loadMessages()
  }
}

const performSearch = async () => {
  if (!selectedContact.value || !searchKeyword.value.trim()) {
    return
  }

  searchMode.value = true
  loading.value = true
  currentSearchKeyword.value = searchKeyword.value

  try {
    const response = await chatApi.searchMessages({
      merge_path: mergePath.value,
      keyword: searchKeyword.value,
      wxid: selectedContact.value.wxid,
      limit: 100
    })
    
    searchResults.value = response.data.messages
  } catch (error) {
    console.error('搜索消息失败:', error)
    alert('搜索失败: ' + (error.response?.data?.error || error.message))
  } finally {
    loading.value = false
  }
}

const clearSearch = () => {
  searchKeyword.value = ''
  currentSearchKeyword.value = ''
  exitSearch()
}

const exitSearch = () => {
  searchMode.value = false
  searchResults.value = []
  currentSearchKeyword.value = ''
}

const onSearchInput = () => {
  if (!searchKeyword.value.trim()) {
    exitSearch()
  }
}

const highlightKeyword = (text, keyword) => {
  if (!keyword || !text) return text
  const regex = new RegExp(`(${keyword})`, 'gi')
  return text.replace(regex, '<mark>$1</mark>')
}

const jumpToMessage = async (message) => {
  // 退出搜索模式
  exitSearch()
  
  // 重新加载消息并跳转到对应消息
  messages.value = []
  currentPage.value = 0
  hasMore.value = true
  
  // 计算目标消息的页码
  const targetPage = Math.floor(message.id / pageSize)
  currentPage.value = targetPage
  
  await loadMessages(true)
  
  // 滚动到目标消息
  setTimeout(() => {
    if (messagesContainer.value) {
      const targetIndex = messages.value.findIndex(m => m.msg_svr_id === message.msg_svr_id)
      if (targetIndex >= 0) {
        const targetElement = messagesContainer.value.children[targetIndex + 1] // +1 for loading div
        if (targetElement) {
          targetElement.scrollIntoView({ behavior: 'smooth', block: 'center' })
        }
      }
    }
  }, 300)
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


.loading,
.loading-top {
  text-align: center;
  padding: 16px;
  color: #666;
  font-size: 14px;
}

.loading-top {
  position: sticky;
  top: 0;
  background: rgba(255, 255, 255, 0.9);
  z-index: 10;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
}

.no-more {
  text-align: center;
  padding: 16px;
  color: #999;
  font-size: 12px;
}
</style>
