<template>
  <div class="app-container" :class="{ 'dark-mode': isDarkMode }">
    <nav class="navbar">
      <div class="nav-container">
        <div class="nav-content">
          <div class="nav-brand">
            <h1 class="brand-title">wx-dump-4</h1>
          </div>
          <button class="mobile-menu-btn" @click="toggleMobileMenu">
            <span></span>
            <span></span>
            <span></span>
          </button>
          <div class="nav-links" :class="{ 'mobile-open': mobileMenuOpen }">
            <router-link to="/" @click="closeMobileMenu">首页</router-link>
            <router-link to="/chat" @click="closeMobileMenu">聊天记录</router-link>
            <router-link to="/contacts" @click="closeMobileMenu">联系人</router-link>
            <router-link to="/favorite" @click="closeMobileMenu">收藏</router-link>
            <router-link to="/moments" @click="closeMobileMenu">朋友圈</router-link>
            <router-link to="/statistics" @click="closeMobileMenu">统计分析</router-link>
            <router-link to="/export" @click="closeMobileMenu">数据导出</router-link>
            <router-link to="/cleanup" @click="closeMobileMenu">清理</router-link>
            <router-link to="/tools" @click="closeMobileMenu">工具</router-link>
            <router-link to="/settings" @click="closeMobileMenu">设置</router-link>
            <button class="theme-toggle" @click="toggleTheme">
              {{ isDarkMode ? '☀️' : '🌙' }}
            </button>
          </div>
        </div>
      </div>
    </nav>
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const isDarkMode = ref(false)
const mobileMenuOpen = ref(false)

const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  localStorage.setItem('darkMode', isDarkMode.value.toString())
  updateTheme()
}

const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}

const closeMobileMenu = () => {
  mobileMenuOpen.value = false
}

const updateTheme = () => {
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

onMounted(() => {
  const saved = localStorage.getItem('darkMode')
  const savedTheme = localStorage.getItem('theme')
  
  if (saved === 'true') {
    isDarkMode.value = true
  } else if (savedTheme === 'dark') {
    isDarkMode.value = true
  } else if (savedTheme === 'auto') {
    // 跟随系统
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    isDarkMode.value = prefersDark
  }
  
  updateTheme()
  
  // 监听主题变化事件
  window.addEventListener('theme-change', (event) => {
    const newTheme = event.detail?.theme
    if (newTheme === 'dark') {
      isDarkMode.value = true
    } else if (newTheme === 'light') {
      isDarkMode.value = false
    } else if (newTheme === 'auto') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      isDarkMode.value = prefersDark
    }
    updateTheme()
  })
  
  // 监听系统主题变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (localStorage.getItem('theme') === 'auto') {
      isDarkMode.value = e.matches
      updateTheme()
    }
  })
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app-container {
  min-height: 100vh;
  background: #f5f5f5;
  transition: background-color 0.3s;
}

.app-container.dark-mode {
  background: #1a1a1a;
  color: #e0e0e0;
}

.navbar {
  background: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.dark-mode .navbar {
  background: #2d2d2d;
  box-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

.nav-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 16px;
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 64px;
}

.nav-brand {
  flex-shrink: 0;
}

.brand-title {
  font-size: 20px;
  font-weight: bold;
  color: #2196f3;
}

.dark-mode .brand-title {
  color: #64b5f6;
}

.mobile-menu-btn {
  display: none;
  flex-direction: column;
  gap: 4px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
}

.mobile-menu-btn span {
  width: 24px;
  height: 2px;
  background: #333;
  transition: all 0.3s;
}

.dark-mode .mobile-menu-btn span {
  background: #e0e0e0;
}

.nav-links {
  display: flex;
  align-items: center;
  gap: 8px;
}

.nav-links a {
  padding: 8px 12px;
  color: #666;
  text-decoration: none;
  border-radius: 4px;
  transition: all 0.2s;
  font-size: 14px;
}

.dark-mode .nav-links a {
  color: #b0b0b0;
}

.nav-links a:hover,
.nav-links a.router-link-active {
  background: #e3f2fd;
  color: #2196f3;
}

.dark-mode .nav-links a:hover,
.dark-mode .nav-links a.router-link-active {
  background: #1e3a5f;
  color: #64b5f6;
}

.theme-toggle {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  padding: 8px;
  margin-left: 8px;
}

.main-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 24px 16px;
}

@media (max-width: 768px) {
  .mobile-menu-btn {
    display: flex;
  }

  .nav-links {
    position: fixed;
    top: 64px;
    left: 0;
    right: 0;
    background: white;
    flex-direction: column;
    align-items: stretch;
    padding: 16px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    transform: translateY(-100%);
    opacity: 0;
    pointer-events: none;
    transition: all 0.3s;
  }

  .dark-mode .nav-links {
    background: #2d2d2d;
  }

  .nav-links.mobile-open {
    transform: translateY(0);
    opacity: 1;
    pointer-events: all;
  }

  .nav-links a {
    width: 100%;
    padding: 12px;
  }

  .main-content {
    padding: 16px;
  }
}
</style>

