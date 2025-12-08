<template>
  <div ref="chartContainer" class="chart-container"></div>
</template>

<script setup>
import { ref, onMounted, watch, onBeforeUnmount } from 'vue'
import * as echarts from 'echarts'

const props = defineProps({
  data: {
    type: Object,
    default: () => ({})
  }
})

const chartContainer = ref(null)
let chartInstance = null

const initChart = () => {
  if (!chartContainer.value) return
  
  chartInstance = echarts.init(chartContainer.value)
  
  const talkers = Object.entries(props.data)
    .sort((a, b) => (b[1].total_count || 0) - (a[1].total_count || 0))
    .slice(0, 10)
  
  const names = talkers.map(([wxid]) => wxid.length > 10 ? wxid.substring(0, 10) + '...' : wxid)
  const values = talkers.map(([, stat]) => stat.total_count || 0)
  
  const option = {
    title: {
      text: '聊天最多的联系人',
      left: 'center',
      textStyle: {
        fontSize: 16
      }
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'shadow'
      },
      formatter: (params) => {
        const param = params[0]
        const originalWxid = talkers[param.dataIndex][0]
        return `${originalWxid}<br/>消息数: ${param.value}`
      }
    },
    xAxis: {
      type: 'category',
      data: names,
      axisLabel: {
        rotate: 45,
        fontSize: 10
      }
    },
    yAxis: {
      type: 'value',
      name: '消息数'
    },
    series: [{
      data: values,
      type: 'bar',
      itemStyle: {
        color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
          { offset: 0, color: '#83bff6' },
          { offset: 0.5, color: '#188df0' },
          { offset: 1, color: '#188df0' }
        ])
      }
    }],
    grid: {
      left: '10%',
      right: '10%',
      bottom: '20%',
      top: '15%'
    }
  }
  
  chartInstance.setOption(option)
  
  // 响应式调整
  window.addEventListener('resize', handleResize)
}

const handleResize = () => {
  if (chartInstance) {
    chartInstance.resize()
  }
}

watch(() => props.data, () => {
  if (chartInstance) {
    initChart()
  }
}, { deep: true })

onMounted(() => {
  initChart()
})

onBeforeUnmount(() => {
  if (chartInstance) {
    window.removeEventListener('resize', handleResize)
    chartInstance.dispose()
  }
})
</script>

<style scoped>
.chart-container {
  width: 100%;
  height: 400px;
}
</style>

