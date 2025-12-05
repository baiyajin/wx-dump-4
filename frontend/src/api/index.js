import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
})

export const wxApi = {
  getWxInfo: () => api.post('/wx/info'),
  decryptDb: (data) => api.post('/wx/decrypt', data),
}

export const chatApi = {
  getContacts: (data) => api.post('/chat/contacts', data),
  getContactDetail: (wxid, data) => api.get(`/chat/contacts/${wxid}`, { data }),
  getMsgCount: (data) => api.post('/chat/msg/count', data),
  getMsgList: (data) => api.post('/chat/msg/list', data),
}

export const favoriteApi = {
  getFavoriteList: (data) => api.post('/favorite/list', data),
  getFavoriteCount: (data) => api.post('/favorite/count', data),
}

export const momentsApi = {
  getMomentsList: (data) => api.post('/moments/list', data),
  getMomentsCount: (data) => api.post('/moments/count', data),
}

export const statisticsApi = {
  getContactStat: (wxid, data) => api.post(`/stat/contact/${wxid}`, data),
  getDateChatStat: (data) => api.post('/stat/date/chat', data),
  getDateHeatmap: (data) => api.post('/stat/date/heatmap', data),
  getTopTalkers: (data) => api.post('/stat/top/talkers', data),
}

export const exportApi = {
  exportCsv: (data) => api.post('/export/csv', data),
  exportJson: (data) => api.post('/export/json', data),
  exportHtml: (data) => api.post('/export/html', data),
}

export const cleanupApi = {
  analyzeStorage: (data) => api.post('/cleanup/analyze', data),
  optimizeDatabase: (data) => api.post('/cleanup/optimize', data),
  vacuumDatabase: (data) => api.post('/cleanup/vacuum', data),
}

export const mergeApi = {
  mergeDatabases: (data) => api.post('/merge/databases', data),
}

