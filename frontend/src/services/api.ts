import axios from 'axios'
import toast from 'react-hot-toast'

const api = axios.create({ baseURL: '/api' })

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})

api.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    } else if (!err.response || err.response.status >= 500) {
      // Falha de rede ou erro do servidor: avisa uma vez só, em qualquer tela.
      // Erros 4xx continuam com cada tela, que sabe a mensagem certa.
      toast.error(err.response ? 'Erro no servidor. Tente de novo.' : 'Sem conexão com o servidor.', { id: 'erro-api' })
    }
    return Promise.reject(err)
  }
)

export default api
