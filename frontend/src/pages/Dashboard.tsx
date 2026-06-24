import { useEffect, useState, useRef } from 'react'
import { Link } from 'react-router-dom'
import { useAuth } from '../contexts/AuthContext'
import api from '../services/api'
import toast from 'react-hot-toast'
import Spinner from '../components/Spinner'
import PageTransition from '../components/PageTransition'

interface Senha {
  id: number
  titulo: string
  login: string
  senha: string
  url?: string
  notas?: string
  categoria: string
  criadoEm: string
}

interface PagedResult {
  items: Senha[]
  page: number
  pageSize: number
  totalCount: number
  totalPages: number
}

const CATEGORIAS = ['Todas', 'Pessoal', 'Trabalho', 'Banco', 'Social', 'Outro']

const categoriaColors: Record<string, string> = {
  Pessoal: 'bg-blue-600',
  Trabalho: 'bg-amber-600',
  Banco: 'bg-green-600',
  Social: 'bg-pink-600',
  Outro: 'bg-gray-600',
}

const PAGE_SIZE = 10

export default function Dashboard() {
  const { logout } = useAuth()
  const [senhas, setSenhas] = useState<Senha[]>([])
  const [filtro, setFiltro] = useState('Todas')
  const [busca, setBusca] = useState('')
  const [showModal, setShowModal] = useState(false)
  const [editId, setEditId] = useState<number | null>(null)
  const [form, setForm] = useState({ titulo: '', login: '', senha: '', url: '', notas: '', categoria: 'Pessoal' })
  const [visibleIds, setVisibleIds] = useState<Set<number>>(new Set())
  const [loading, setLoading] = useState(true)
  const [page, setPage] = useState(1)
  const [totalPages, setTotalPages] = useState(1)
  const [totalCount, setTotalCount] = useState(0)
  const fileRef = useRef<HTMLInputElement>(null)

  const load = async (p = page) => {
    setLoading(true)
    const { data } = await api.get<PagedResult>('/senhas', { params: { page: p, pageSize: PAGE_SIZE } })
    setSenhas(data.items)
    setTotalPages(data.totalPages)
    setTotalCount(data.totalCount)
    setPage(data.page)
    setLoading(false)
  }

  useEffect(() => { load(1) }, [])

  const filtered = senhas
    .filter(s => filtro === 'Todas' || s.categoria === filtro)
    .filter(s => !busca || s.titulo.toLowerCase().includes(busca.toLowerCase()) || s.login.toLowerCase().includes(busca.toLowerCase()) || (s.url || '').toLowerCase().includes(busca.toLowerCase()))

  const toggleVisible = (id: number) => {
    setVisibleIds((prev) => { const next = new Set(prev); next.has(id) ? next.delete(id) : next.add(id); return next })
  }

  const copy = (text: string) => { navigator.clipboard.writeText(text); toast.success('Copiado!') }

  const openNew = () => { setEditId(null); setForm({ titulo: '', login: '', senha: '', url: '', notas: '', categoria: 'Pessoal' }); setShowModal(true) }

  const openEdit = (s: Senha) => { setEditId(s.id); setForm({ titulo: s.titulo, login: s.login, senha: s.senha, url: s.url || '', notas: s.notas || '', categoria: s.categoria }); setShowModal(true) }

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault()
    if (editId) { await api.put(`/senhas/${editId}`, form); toast.success('Atualizado!') }
    else { await api.post('/senhas', form); toast.success('Criado!') }
    setShowModal(false); load()
  }

  const handleDelete = async (id: number) => { if (!confirm('Deletar essa senha?')) return; await api.delete(`/senhas/${id}`); toast.success('Deletado!'); load() }

  const exportJson = async () => {
    const { data } = await api.get('/senhas/export/json', { responseType: 'blob' })
    const url = URL.createObjectURL(data)
    const a = document.createElement('a'); a.href = url; a.download = 'cofre-senhas-export.json'; a.click()
    toast.success('Exportado!')
  }

  const exportCsv = async () => {
    const { data } = await api.get('/senhas/export/csv', { responseType: 'blob' })
    const url = URL.createObjectURL(data)
    const a = document.createElement('a'); a.href = url; a.download = 'cofre-senhas-export.csv'; a.click()
    toast.success('Exportado!')
  }

  const importJson = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return
    const text = await file.text()
    try {
      const parsed = JSON.parse(text)
      const senhasToImport = parsed.map((s: Senha) => ({ titulo: s.titulo, login: s.login, senha: s.senha, url: s.url, notas: s.notas, categoria: s.categoria || 'Pessoal' }))
      const { data } = await api.post('/senhas/import/json', senhasToImport)
      toast.success(`${data.imported} senhas importadas!`)
      load()
    } catch { toast.error('Arquivo inválido') }
    e.target.value = ''
  }

  const goToPage = (p: number) => { if (p >= 1 && p <= totalPages) load(p) }

  return (
    <PageTransition>
    <div className="min-h-screen bg-gray-900 p-6">
      <header className="flex justify-between items-center mb-6 max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold">🔐 Minhas Senhas</h1>
        <div className="flex gap-3">
          <Link to="/generator" className="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg text-sm transition">Gerador</Link>
          <Link to="/profile" className="px-4 py-2 bg-gray-600 hover:bg-gray-500 rounded-lg text-sm transition">👤</Link>
          <Link to="/settings" className="px-4 py-2 bg-gray-600 hover:bg-gray-500 rounded-lg text-sm transition">⚙️</Link>
          <button onClick={logout} className="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-sm transition">Sair</button>
        </div>
      </header>

      <div className="max-w-4xl mx-auto">
        {/* Busca */}
        <input
          type="text"
          placeholder="🔍 Buscar por título, login ou URL..."
          value={busca}
          onChange={(e) => setBusca(e.target.value)}
          className="w-full p-3 mb-4 bg-gray-800 rounded-lg outline-none focus:ring-2 focus:ring-blue-500"
        />

        {/* Filtro por categoria */}
        <div className="flex gap-2 mb-4 flex-wrap">
          {CATEGORIAS.map((cat) => (
            <button key={cat} onClick={() => setFiltro(cat)} className={`px-3 py-1 rounded-full text-sm transition ${filtro === cat ? 'bg-blue-500 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}>
              {cat}
            </button>
          ))}
        </div>

        {/* Ações */}
        <div className="flex gap-2 mb-6 flex-wrap">
          <button onClick={openNew} className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition">+ Nova</button>
          <button onClick={exportJson} className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg text-sm transition">⬇ JSON</button>
          <button onClick={exportCsv} className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg text-sm transition">⬇ CSV</button>
          <button onClick={() => fileRef.current?.click()} className="px-4 py-2 bg-teal-600 hover:bg-teal-700 rounded-lg text-sm transition">⬆ Importar</button>
          <input ref={fileRef} type="file" accept=".json" onChange={importJson} className="hidden" />
          <span className="ml-auto text-gray-400 text-sm self-center">{totalCount} senha{totalCount !== 1 ? 's' : ''}</span>
        </div>

        {loading ? (
          <Spinner />
        ) : filtered.length === 0 ? (
          <p className="text-gray-400 text-center mt-10">Nenhuma senha encontrada.</p>
        ) : (
          <div className="grid gap-4">
            {filtered.map((s) => (
              <div key={s.id} className="bg-gray-800 p-4 rounded-xl flex justify-between items-center">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="font-semibold text-lg">{s.titulo}</h3>
                    <span className={`px-2 py-0.5 rounded text-xs font-medium ${categoriaColors[s.categoria] || 'bg-gray-600'}`}>{s.categoria}</span>
                  </div>
                  <p className="text-gray-400 text-sm">{s.login}</p>
                  <p className="text-sm mt-1 font-mono">{visibleIds.has(s.id) ? s.senha : '••••••••••'}</p>
                  {s.url && <p className="text-gray-500 text-xs mt-1">{s.url}</p>}
                </div>
                <div className="flex gap-2 ml-4">
                  <button onClick={() => toggleVisible(s.id)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Ver">👁</button>
                  <button onClick={() => copy(s.senha)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Copiar">📋</button>
                  <button onClick={() => openEdit(s)} className="p-2 bg-gray-700 rounded hover:bg-gray-600" title="Editar">✏️</button>
                  <button onClick={() => handleDelete(s.id)} className="p-2 bg-gray-700 rounded hover:bg-red-600" title="Deletar">🗑️</button>
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Paginação */}
        {totalPages > 1 && (
          <div className="flex justify-center items-center gap-2 mt-6">
            <button onClick={() => goToPage(page - 1)} disabled={page === 1} className="px-3 py-2 bg-gray-700 rounded-lg disabled:opacity-40 hover:bg-gray-600 transition">←</button>
            {Array.from({ length: totalPages }, (_, i) => i + 1)
              .filter(p => p === 1 || p === totalPages || Math.abs(p - page) <= 1)
              .map((p, idx, arr) => (
                <span key={p}>
                  {idx > 0 && arr[idx - 1] !== p - 1 && <span className="text-gray-500 px-1">...</span>}
                  <button onClick={() => goToPage(p)} className={`px-3 py-2 rounded-lg transition ${p === page ? 'bg-blue-600' : 'bg-gray-700 hover:bg-gray-600'}`}>{p}</button>
                </span>
              ))}
            <button onClick={() => goToPage(page + 1)} disabled={page === totalPages} className="px-3 py-2 bg-gray-700 rounded-lg disabled:opacity-40 hover:bg-gray-600 transition">→</button>
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
            <select value={form.categoria} onChange={(e) => setForm({ ...form, categoria: e.target.value })} className="w-full p-3 mb-3 bg-gray-700 rounded-lg outline-none">
              {CATEGORIAS.filter(c => c !== 'Todas').map(c => <option key={c} value={c}>{c}</option>)}
            </select>
            <textarea placeholder="Notas (opcional)" value={form.notas} onChange={(e) => setForm({ ...form, notas: e.target.value })} className="w-full p-3 mb-4 bg-gray-700 rounded-lg outline-none resize-none" rows={2} />
            <div className="flex gap-3">
              <button type="submit" className="flex-1 p-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition">Salvar</button>
              <button type="button" onClick={() => setShowModal(false)} className="flex-1 p-3 bg-gray-600 hover:bg-gray-500 rounded-lg transition">Cancelar</button>
            </div>
          </form>
        </div>
      )}
    </div>
    </PageTransition>
  )
}
