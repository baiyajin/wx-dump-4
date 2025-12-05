import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
})

export const wxApi = {
  getWxInfo: () => api.post('/wx/info'),
  decryptDb: (data) => api.post('/wx/decrypt', data),
}

