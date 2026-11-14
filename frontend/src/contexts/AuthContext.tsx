import { createContext, useContext, useEffect, useState, ReactNode } from 'react'
import api from '../services/api'
import toast from 'react-hot-toast'

interface AuthContextType {
  token: string | null
  login: (email: string, senha: string, totpCode?: string) => Promise<{ twoFactorRequired?: boolean } | void>
  register: (nome: string, email: string, senha: string) => Promise<void>
  logout: () => void
}

const INATIVIDADE_MS = 15 * 60 * 1000

const AuthContext = createContext<AuthContextType>(null!)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'))

  const login = async (email: string, senha: string, totpCode?: string) => {
    const { data } = await api.post('/auth/login', { email, senha, totpCode })
    if (data.twoFactorRequired) return { twoFactorRequired: true }
    localStorage.setItem('token', data.token)
    setToken(data.token)
  }

  const register = async (nome: string, email: string, senha: string) => {
    const { data } = await api.post('/auth/register', { nome, email, senha })
    localStorage.setItem('token', data.token)
    setToken(data.token)
    toast.success('Conta criada!')
  }

  const logout = () => {
    // Invalida o token no servidor (melhor esforço) e limpa o local.
    api.post('/auth/logout').catch(() => {})
    localStorage.removeItem('token')
    setToken(null)
  }

  // Bloqueio por inatividade: sem interação por 15 min, encerra a sessão.
  useEffect(() => {
    if (!token) return
    let timer = window.setTimeout(logout, INATIVIDADE_MS)
    const reiniciar = () => {
      window.clearTimeout(timer)
      timer = window.setTimeout(logout, INATIVIDADE_MS)
    }
    const eventos = ['mousemove', 'keydown', 'click', 'touchstart']
    eventos.forEach((e) => window.addEventListener(e, reiniciar))
    return () => {
      window.clearTimeout(timer)
      eventos.forEach((e) => window.removeEventListener(e, reiniciar))
    }
  }, [token])

  return (
    <AuthContext.Provider value={{ token, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  )
}

export const useAuth = () => useContext(AuthContext)
