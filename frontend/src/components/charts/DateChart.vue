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
  
  const dates = Object.keys(props.data).sort()
  const values = dates.map(date => props.data[date]?.total_count || 0)
  
  const option = {
    title: {
      text: '日期聊天统计',
      left: 'center',
      textStyle: {
        fontSize: 16
      }
    },
    tooltip: {
      trigger: 'axis',
      formatter: (params) => {
        const param = params[0]
        return `${param.name}<br/>消息数: ${param.value}`
      }
    },
    xAxis: {
      type: 'category',
      data: dates,
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
      type: 'line',
      smooth: true,
      areaStyle: {
        opacity: 0.3
      },
      itemStyle: {
        color: '#2196f3'
      }
    }],
    grid: {
      left: '10%',
      right: '10%',
      bottom: '15%',
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

