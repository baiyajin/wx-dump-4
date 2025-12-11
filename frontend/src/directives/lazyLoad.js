/**
 * 图片懒加载指令
 * 使用 Intersection Observer API 实现
 */
export default {
  mounted(el, binding) {
    const imageObserver = new IntersectionObserver((entries, observer) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          const img = entry.target
          const src = binding.value || img.dataset.src
          
          if (src) {
            img.src = src
            img.classList.add('loaded')
            observer.unobserve(img)
          }
        }
      })
    }, {
      rootMargin: '50px' // 提前50px开始加载
    })
    
    // 设置占位图
    if (el.tagName === 'IMG') {
      el.dataset.src = binding.value
      el.src = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" width="200" height="200"%3E%3Crect width="200" height="200" fill="%23f0f0f0"/%3E%3C/svg%3E'
      imageObserver.observe(el)
    }
  },
  
  updated(el, binding) {
    if (binding.value !== binding.oldValue) {
      el.dataset.src = binding.value
    }
  }
}
