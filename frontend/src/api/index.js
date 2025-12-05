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

