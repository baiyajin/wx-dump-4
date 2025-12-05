<template>
  <div class="statistics-view">
    <h2 class="text-2xl font-bold mb-6">统计分析</h2>

    <div class="stats-container">
      <!-- 统计配置 -->
      <div class="stats-config">
        <div class="config-section">
          <label class="config-label">数据库路径</label>
          <input
            v-model="mergePath"
            type="text"
            placeholder="请输入合并后的数据库路径"
            class="config-input"
          />
        </div>

        <div class="config-section">
          <label class="config-label">联系人（可选）</label>
          <input
            v-model="wxid"
            type="text"
            placeholder="留空则统计所有联系人"
            class="config-input"
          />
        </div>

        <button
          @click="loadStatistics"
          :disabled="loading || !mergePath"
          class="load-button"
        >
          {{ loading ? '加载中...' : '加载统计' }}
        </button>
      </div>

      <!-- 统计结果 -->
      <div v-if="statData" class="stats-results">
        <!-- 联系人统计 -->
        <div v-if="contactStat" class="stat-card">
          <h3 class="stat-title">联系人统计</h3>
          <div class="stat-content">
            <div class="stat-item">
              <span class="stat-label">总消息数：</span>
              <span class="stat-value">{{ contactStat.total_count }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">发送消息：</span>
              <span class="stat-value">{{ contactStat.sender_count }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">接收消息：</span>
              <span class="stat-value">{{ contactStat.receiver_count }}</span>
            </div>
          </div>
        </div>

        <!-- 日期统计 -->
        <div v-if="dateStats && Object.keys(dateStats).length > 0" class="stat-card">
          <h3 class="stat-title">日期统计</h3>
          <div class="date-stats-list">
            <div
              v-for="(stat, date) in dateStats"
              :key="date"
              class="date-stat-item"
            >
              <span class="date-label">{{ date }}</span>
              <span class="date-value">{{ stat.total_count }} 条</span>
            </div>
          </div>
        </div>

        <!-- 热力图数据 -->
        <div v-if="heatmapData && heatmapData.length > 0" class="stat-card">
          <h3 class="stat-title">聊天热力图</h3>
          <div class="heatmap-container">
            <div
              v-for="item in heatmapData"
              :key="item.date"
              class="heatmap-item"
              :style="{ opacity: getHeatmapOpacity(item.count) }"
              :title="`${item.date}: ${item.count} 条消息`"
            >
              {{ item.count }}
            </div>
          </div>
        </div>

        <!-- 聊天最多的联系人 -->
        <div v-if="topTalkers && Object.keys(topTalkers).length > 0" class="stat-card">
          <h3 class="stat-title">聊天最多的联系人</h3>
          <div class="talkers-list">
            <div
              v-for="(stat, wxid) in topTalkers"
              :key="wxid"
              class="talker-item"
            >
              <span class="talker-wxid">{{ wxid }}</span>
              <span class="talker-count">{{ stat.total_count }} 条</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { statisticsApi } from '../api/index.js'

const mergePath = ref('')
const wxid = ref('')
const loading = ref(false)
const statData = ref(null)
const contactStat = ref(null)
const dateStats = ref(null)
const heatmapData = ref(null)
const topTalkers = ref(null)

const loadStatistics = async () => {
  if (!mergePath.value) {
    alert('请输入数据库路径')
    return
  }

  loading.value = true
  statData.value = null
  contactStat.value = null
  dateStats.value = null
  heatmapData.value = null
  topTalkers.value = null

  try {
    // 加载联系人统计
    if (wxid.value) {
      const contactResponse = await statisticsApi.getContactStat(wxid.value, {
        merge_path: mergePath.value,
        wxid: wxid.value
      })
      contactStat.value = contactResponse.data
    }

    // 加载日期统计
    const dateResponse = await statisticsApi.getDateChatStat({
      merge_path: mergePath.value,
      wxid: wxid.value || null
    })
    dateStats.value = dateResponse.data?.stats || {}

    // 加载热力图数据
    const heatmapResponse = await statisticsApi.getDateHeatmap({
      merge_path: mergePath.value,
      wxid: wxid.value || null
    })
    heatmapData.value = heatmapResponse.data?.data || []

    // 加载聊天最多的联系人
    const talkersResponse = await statisticsApi.getTopTalkers({
      merge_path: mergePath.value,
      top: 10
    })
    topTalkers.value = talkersResponse.data?.talkers || {}

    statData.value = true
  } catch (error) {
    console.error('加载统计失败:', error)
    alert('加载统计失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

const getHeatmapOpacity = (count) => {
  if (!count) return 0.1
  const max = Math.max(...heatmapData.value.map(item => item.count))
  return Math.min(0.3 + (count / max) * 0.7, 1)
}

onMounted(() => {
  mergePath.value = localStorage.getItem('merge_path') || ''
})
</script>

<style scoped>
.statistics-view {
  padding: 24px;
}

.stats-container {
  max-width: 1200px;
}

.stats-config {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin-bottom: 24px;
}

.config-section {
  margin-bottom: 20px;
}

.config-label {
  display: block;
  font-weight: 500;
  margin-bottom: 8px;
  color: #333;
}

.config-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.load-button {
  width: 100%;
  padding: 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
}

.load-button:hover:not(:disabled) {
  background: #1976d2;
}

.load-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.stats-results {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
}

.stat-card {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.stat-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #333;
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.stat-label {
  color: #666;
}

.stat-value {
  font-weight: 600;
  color: #2196f3;
}

.date-stats-list {
  max-height: 400px;
  overflow-y: auto;
}

.date-stat-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.date-label {
  color: #666;
}

.date-value {
  font-weight: 500;
}

.heatmap-container {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.heatmap-item {
  aspect-ratio: 1;
  background: #2196f3;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  color: white;
  cursor: pointer;
}

.talkers-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.talker-item {
  display: flex;
  justify-content: space-between;
  padding: 8px;
  background: #f5f5f5;
  border-radius: 4px;
}

.talker-wxid {
  color: #333;
  word-break: break-all;
}

.talker-count {
  font-weight: 600;
  color: #2196f3;
  margin-left: 12px;
}
</style>

