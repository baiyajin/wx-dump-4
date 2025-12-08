<template>
  <div class="contacts-view">
    <h2 class="text-2xl font-bold mb-6">联系人</h2>

    <div class="contacts-container">
      <!-- 搜索和筛选 -->
      <div class="contacts-header">
        <input
          v-model="searchKeyword"
          type="text"
          placeholder="搜索联系人..."
          class="search-input"
        />
        <select v-model="filterType" class="filter-select">
          <option value="all">全部</option>
          <option value="friend">好友</option>
          <option value="group">群聊</option>
          <option value="public">公众号</option>
        </select>
      </div>

      <!-- 联系人列表 -->
      <div class="contacts-list">
        <div
          v-for="contact in filteredContacts"
          :key="contact.wxid"
          class="contact-card"
          @click="viewContactDetail(contact)"
        >
          <div class="contact-avatar">{{ getAvatarText(contact) }}</div>
          <div class="contact-details">
            <div class="contact-name">
              {{ contact.remark || contact.nickname || contact.wxid }}
            </div>
            <div class="contact-wxid">{{ contact.wxid }}</div>
            <div v-if="contact.account" class="contact-account">
              账号: {{ contact.account }}
            </div>
          </div>
        </div>
      </div>

      <div v-if="loading" class="loading">加载中...</div>
      <div v-if="error" class="error">{{ error }}</div>
    </div>

    <!-- 联系人详情弹窗 -->
    <div v-if="showDetail && selectedContact" class="detail-modal" @click.self="closeDetail">
      <div class="detail-content">
        <div class="detail-header">
          <h3>联系人详情</h3>
          <button @click="closeDetail" class="close-btn">×</button>
        </div>
        <div class="detail-body">
          <div class="detail-avatar-large">
            {{ getAvatarText(selectedContact) }}
          </div>
          <div class="detail-info">
            <div class="detail-item">
              <label>微信ID:</label>
              <span>{{ selectedContact.wxid }}</span>
            </div>
            <div v-if="selectedContact.nickname" class="detail-item">
              <label>昵称:</label>
              <span>{{ selectedContact.nickname }}</span>
            </div>
            <div v-if="selectedContact.remark" class="detail-item">
              <label>备注:</label>
              <span>{{ selectedContact.remark }}</span>
            </div>
            <div v-if="selectedContact.account" class="detail-item">
              <label>账号:</label>
              <span>{{ selectedContact.account }}</span>
            </div>
            <div v-if="selectedContact.msg_count !== undefined" class="detail-item">
              <label>消息数:</label>
              <span>{{ selectedContact.msg_count }}</span>
            </div>
          </div>
          <div class="detail-actions">
            <button @click="$router.push(`/chat?wxid=${selectedContact.wxid}`)" class="btn-primary">
              查看聊天记录
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { chatApi } from '../api/index.js'
import { chatApi as contactApi } from '../api/index.js'

const searchKeyword = ref('')
const filterType = ref('all')
const contacts = ref([])
const loading = ref(false)
const error = ref(null)
const mergePath = ref('')

const filteredContacts = computed(() => {
  let result = contacts.value

  // 搜索过滤
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    result = result.filter(contact =>
      (contact.nickname && contact.nickname.toLowerCase().includes(keyword)) ||
      (contact.remark && contact.remark.toLowerCase().includes(keyword)) ||
      contact.wxid.toLowerCase().includes(keyword) ||
      (contact.account && contact.account.toLowerCase().includes(keyword))
    )
  }

  // 类型过滤
  if (filterType.value !== 'all') {
    // 这里需要根据contact_type进行过滤
    // 暂时先返回所有结果
  }

  return result
})

const getAvatarText = (contact) => {
  const name = contact.remark || contact.nickname || contact.wxid
  return name.charAt(0).toUpperCase()
}

const selectedContact = ref(null)
const showDetail = ref(false)

const viewContactDetail = async (contact) => {
  selectedContact.value = contact
  showDetail.value = true
  
  // 获取联系人详情
  try {
    const response = await chatApi.getContactDetail(contact.wxid, {
      merge_path: mergePath.value
    })
    if (response.data) {
      selectedContact.value = { ...contact, ...response.data }
    }
  } catch (err) {
    console.error('获取联系人详情失败:', err)
  }
}

const closeDetail = () => {
  showDetail.value = false
  selectedContact.value = null
}

const loadContacts = async () => {
  if (!mergePath.value) {
    error.value = '请先在设置中配置数据库路径'
    return
  }

  loading.value = true
  error.value = null

  try {
    // 这里需要调用联系人API，暂时使用聊天记录API获取联系人
    const response = await chatApi.getContacts({
      merge_path: mergePath.value
    })
    
    // 从聊天记录中提取联系人信息
    contacts.value = response.data.contacts.map(c => ({
      wxid: c.wxid,
      nickname: null,
      remark: null,
      account: null,
      msg_count: c.msg_count
    }))
  } catch (err) {
    error.value = '加载联系人失败: ' + err.message
    console.error('Error loading contacts:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
  if (mergePath.value) {
    loadContacts()
  }
})
</script>

<style scoped>
.contacts-view {
  padding: 24px;
}

.contacts-container {
  max-width: 1200px;
}

.contacts-header {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.search-input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.filter-select {
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.contacts-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.contact-card {
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  display: flex;
  align-items: center;
  gap: 16px;
  cursor: pointer;
  transition: transform 0.2s;
}

.contact-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.15);
}

.contact-avatar {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: #2196f3;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
  flex-shrink: 0;
}

.contact-details {
  flex: 1;
  min-width: 0;
}

.contact-name {
  font-weight: 500;
  font-size: 16px;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.contact-wxid {
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.contact-account {
  font-size: 12px;
  color: #999;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loading,
.error {
  text-align: center;
  padding: 24px;
  color: #666;
}

.error {
  color: #f44336;
}

.detail-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.detail-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  overflow-y: auto;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
}

.detail-header h3 {
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #666;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  color: #000;
}

.detail-body {
  padding: 24px;
}

.detail-avatar-large {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: #2196f3;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: bold;
  margin: 0 auto 24px;
}

.detail-info {
  margin-bottom: 24px;
}

.detail-item {
  display: flex;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.detail-item label {
  font-weight: 500;
  width: 80px;
  color: #666;
}

.detail-item span {
  flex: 1;
}

.detail-actions {
  display: flex;
  gap: 12px;
}

.btn-primary {
  flex: 1;
  padding: 10px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary:hover {
  background: #1976d2;
}
</style>

