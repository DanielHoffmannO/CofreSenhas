import { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../contexts/AuthContext'
import toast from 'react-hot-toast'

export default function Login() {
  const [email, setEmail] = useState('')
  const [senha, setSenha] = useState('')
  const { login } = useAuth()
  const navigate = useNavigate()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await login(email, senha)
      navigate('/')
    } catch {
      toast.error('Email ou senha inválidos')
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-900">
      <form onSubmit={handleSubmit} className="bg-gray-800 p-8 rounded-xl shadow-lg w-full max-w-sm">
        <h1 className="text-2xl font-bold text-center mb-6">🔐 Cofre de Senhas</h1>
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="w-full p-3 mb-4 bg-gray-700 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
          required
        />
        <input
          type="password"
          placeholder="Senha"
          value={senha}
          onChange={(e) => setSenha(e.target.value)}
          className="w-full p-3 mb-6 bg-gray-700 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
          required
        />
        <button type="submit" className="w-full p-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition">
          Entrar
        </button>
        <p className="text-center mt-4 text-gray-400 text-sm">
          Não tem conta? <Link to="/register" className="text-blue-400 hover:underline">Criar conta</Link>
        </p>
      </form>
    </div>
  )
}
