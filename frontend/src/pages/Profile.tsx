import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import api from '../services/api'
import toast from 'react-hot-toast'
import PageTransition from '../components/PageTransition'

interface Profile {
  id: number
  nome: string
  email: string
  criadoEm: string
  twoFactorEnabled: boolean
  masterPasswordConfigured: boolean
}

export default function Profile() {
  const [profile, setProfile] = useState<Profile | null>(null)
  const [form, setForm] = useState({ senhaAtual: '', novaSenha: '', confirmarSenha: '' })
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    api.get<Profile>('/auth/profile').then(({ data }) => setProfile(data))
  }, [])

  const handleChangePassword = async (e: React.FormEvent) => {
    e.preventDefault()
    if (form.novaSenha !== form.confirmarSenha) {
      toast.error('As senhas não coincidem')
      return
    }
    if (form.novaSenha.length < 6) {
      toast.error('A nova senha deve ter pelo menos 6 caracteres')
      return
    }
    setLoading(true)
    try {
      await api.put('/auth/change-password', { senhaAtual: form.senhaAtual, novaSenha: form.novaSenha })
      toast.success('Senha alterada com sucesso!')
      setForm({ senhaAtual: '', novaSenha: '', confirmarSenha: '' })
    } catch (err: any) {
      toast.error(err.response?.data?.message || 'Erro ao alterar senha')
    } finally {
      setLoading(false)
    }
  }

  return (
    <PageTransition>
    <div className="min-h-screen bg-gray-900 p-6">
      <div className="max-w-md mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">👤 Perfil</h1>
          <Link to="/" className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm transition">← Voltar</Link>
        </div>

        {/* Info do usuário */}
        {profile && (
          <div className="bg-gray-800 p-6 rounded-xl mb-6">
            <h2 className="text-lg font-semibold mb-4">Informações</h2>
            <div className="space-y-3 text-sm">
              <div className="flex justify-between">
                <span className="text-gray-400">Nome</span>
                <span>{profile.nome}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Email</span>
                <span>{profile.email}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Membro desde</span>
                <span>{new Date(profile.criadoEm).toLocaleDateString('pt-BR')}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">2FA</span>
                <span className={profile.twoFactorEnabled ? 'text-green-400' : 'text-gray-500'}>{profile.twoFactorEnabled ? '✅ Ativado' : '❌ Desativado'}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Master Password</span>
                <span className={profile.masterPasswordConfigured ? 'text-green-400' : 'text-gray-500'}>{profile.masterPasswordConfigured ? '✅ Configurada' : '❌ Não configurada'}</span>
              </div>
            </div>
          </div>
        )}

        {/* Trocar senha */}
        <div className="bg-gray-800 p-6 rounded-xl">
          <h2 className="text-lg font-semibold mb-4">🔑 Alterar Senha</h2>
          <form onSubmit={handleChangePassword} className="space-y-3">
            <input
              type="password"
              placeholder="Senha atual"
              value={form.senhaAtual}
              onChange={(e) => setForm({ ...form, senhaAtual: e.target.value })}
              className="w-full p-3 bg-gray-700 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
            <input
              type="password"
              placeholder="Nova senha"
              value={form.novaSenha}
              onChange={(e) => setForm({ ...form, novaSenha: e.target.value })}
              className="w-full p-3 bg-gray-700 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
            <input
              type="password"
              placeholder="Confirmar nova senha"
              value={form.confirmarSenha}
              onChange={(e) => setForm({ ...form, confirmarSenha: e.target.value })}
              className="w-full p-3 bg-gray-700 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
            <button
              type="submit"
              disabled={loading}
              className="w-full p-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition disabled:opacity-50"
            >
              {loading ? 'Alterando...' : 'Alterar Senha'}
            </button>
          </form>
        </div>
      </div>
    </div>
    </PageTransition>
  )
}
