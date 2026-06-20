import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { useAuth } from '../contexts/AuthContext'
import api from '../services/api'
import toast from 'react-hot-toast'

interface Senha {
  id: number
  titulo: string
  login: string
  senha: string
  url?: string
  notas?: string
  criadoEm: string
}

export default function Dashboard() {
  const { logout } = useAuth()
  const [senhas, setSenhas] = useState<Senha[]>([])
  const [showModal, setShowModal] = useState(false)
  const [editId, setEditId] = useState<number | null>(null)
  const [form, setForm] = useState({ titulo: '', login: '', senha: '', url: '', notas: '' })
  const [visibleIds, setVisibleIds] = useState<Set<number>>(new Set())

  const load = async () => {
    const { data } = await api.get('/senhas')
    setSenhas(data)
  }

  useEffect(() => { load() }, [])

  const toggleVisible = (id: number) => {
    setVisibleIds((prev) => {
      const next = new Set(prev)
      next.has(id) ? next.delete(id) : next.add(id)
      return next
    })
  }

  const copy = (text: string) => {
    navigator.clipboard.writeText(text)
    toast.success('Copiado!')
  }

  const openNew = () => {
    setEditId(null)
    setForm({ titulo: '', login: '', senha: '', url: '', notas: '' })
    setShowModal(true)
  }

  const openEdit = (s: Senha) => {
    setEditId(s.id)
    setForm({ titulo: s.titulo, login: s.login, senha: s.senha, url: s.url || '', notas: s.notas || '' })
    setShowModal(true)
  }

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault()
    if (editId) {
      await api.put(`/senhas/${editId}`, form)
      toast.success('Atualizado!')
    } else {
      await api.post('/senhas', form)
      toast.success('Criado!')
    }
    setShowModal(false)
    load()
  }

  const handleDelete = async (id: number) => {
    if (!confirm('Deletar essa senha?')) return
    await api.delete(`/senhas/${id}`)
    toast.success('Deletado!')
    load()
  }

  return (
    <div className="min-h-screen bg-gray-900 p-6">
      <header className="flex justify-between items-center mb-8 max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold">🔐 Minhas Senhas</h1>
        <div className="flex gap-3">
          <Link to="/generator" className="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg text-sm transition">
            Gerador
          </Link>
          <button onClick={logout} className="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-sm transition">
            Sair
          </button>
        </div>
      </header>

      <div className="max-w-4xl mx-auto">
        <button onClick={openNew} className="mb-6 px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition">
          + Nova Senha
        </button>

        {senhas.length === 0 ? (
          <p className="text-gray-400 text-center mt-10">Nenhuma senha cadastrada ainda.</p>
        ) : (
          <div className="grid gap-4">
            {senhas.map((s) => (
              <div key={s.id} className="bg-gray-800 p-4 rounded-xl flex justify-between items-center">
                <div className="flex-1">
                  <h3 className="font-semibold text-lg">{s.titulo}</h3>
                  <p className="text-gray-400 text-sm">{s.login}</p>
                  <p className="text-sm mt-1 font-mono">
                    {visibleIds.has(s.id) ? s.senha : '••••••••••'}
                  </p>
                  {s.url && <p className="text-gray-500 text-xs mt-1">{s.url}</p>}
                </div>
                <div className="flex gap-2 ml-4">
                  <button onClick={() => toggleVisible(s.id)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Ver">
                    👁
                  </button>
                  <button onClick={() => copy(s.senha)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Copiar">
                    📋
                  </button>
                  <button onClick={() => openEdit(s)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Editar">
                    ✏️
                  </button>
                  <button onClick={() => handleDelete(s.id)} className="p-2 bg-gray-700 rounded hover:bg-red-600" title="Deletar">
                    🗑️
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
          <form onSubmit={handleSave} className="bg-gray-800 p-6 rounded-xl w-full max-w-md">
            <h2 className="text-xl font-bold mb-4">{editId ? 'Editar' : 'Nova'} Senha</h2>
            <input placeholder="Título" value={form.titulo} onChange={(e) => setForm({ ...form, titulo: e.target.value })} className="w-full p-3 mb-3 bg-gray-700 rounded-lg outline-none" required />
            <input placeholder="Login/Email" value={form.login} onChange={(e) => setForm({ ...form, login: e.target.value })} className="w-full p-3 mb-3 bg-gray-700 rounded-lg outline-none" required />
            <input placeholder="Senha" value={form.senha} onChange={(e) => setForm({ ...form, senha: e.target.value })} className="w-full p-3 mb-3 bg-gray-700 rounded-lg outline-none" required />
            <input placeholder="URL (opcional)" value={form.url} onChange={(e) => setForm({ ...form, url: e.target.value })} className="w-full p-3 mb-3 bg-gray-700 rounded-lg outline-none" />
            <textarea placeholder="Notas (opcional)" value={form.notas} onChange={(e) => setForm({ ...form, notas: e.target.value })} className="w-full p-3 mb-4 bg-gray-700 rounded-lg outline-none resize-none" rows={2} />
            <div className="flex gap-3">
              <button type="submit" className="flex-1 p-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition">Salvar</button>
              <button type="button" onClick={() => setShowModal(false)} className="flex-1 p-3 bg-gray-600 hover:bg-gray-500 rounded-lg transition">Cancelar</button>
            </div>
          </form>
        </div>
      )}
    </div>
  )
}
