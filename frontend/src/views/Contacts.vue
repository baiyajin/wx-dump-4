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

const viewContactDetail = (contact) => {
  // 跳转到聊天页面或显示详情
  console.log('View contact:', contact)
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
</style>

