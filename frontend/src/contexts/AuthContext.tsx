import { createContext, useContext, useState, ReactNode } from 'react'
import api from '../services/api'
import toast from 'react-hot-toast'

interface AuthContextType {
  token: string | null
  login: (email: string, senha: string) => Promise<void>
  register: (nome: string, email: string, senha: string) => Promise<void>
  logout: () => void
}

const AuthContext = createContext<AuthContextType>(null!)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'))

  const login = async (email: string, senha: string) => {
    const { data } = await api.post('/auth/login', { email, senha })
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
    localStorage.removeItem('token')
    setToken(null)
  }

  return (
    <AuthContext.Provider value={{ token, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  )
}

export const useAuth = () => useContext(AuthContext)
