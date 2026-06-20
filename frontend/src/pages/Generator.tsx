import { useState } from 'react'
import { Link } from 'react-router-dom'
import api from '../services/api'
import toast from 'react-hot-toast'

const forcaColors: Record<string, string> = {
  Fraca: 'bg-red-500',
  Media: 'bg-yellow-500',
  Forte: 'bg-green-500',
  MuitoForte: 'bg-emerald-400',
}

export default function Generator() {
  const [tamanho, setTamanho] = useState(16)
  const [maiusculas, setMaiusculas] = useState(true)
  const [numeros, setNumeros] = useState(true)
  const [especiais, setEspeciais] = useState(true)
  const [resultado, setResultado] = useState<{ senha: string; forca: string } | null>(null)

  const gerar = async () => {
    const { data } = await api.post('/gerador', {
      tamanho,
      usarMaiusculas: maiusculas,
      usarNumeros: numeros,
      usarEspeciais: especiais,
    })
    setResultado(data)
  }

  const copiar = () => {
    if (resultado) {
      navigator.clipboard.writeText(resultado.senha)
      toast.success('Senha copiada!')
    }
  }

  return (
    <div className="min-h-screen bg-gray-900 p-6">
      <div className="max-w-md mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">⚡ Gerador de Senhas</h1>
          <Link to="/" className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm transition">
            ← Voltar
          </Link>
        </div>

        <div className="bg-gray-800 p-6 rounded-xl">
          <label className="block mb-4">
            <span className="text-sm text-gray-400">Tamanho: {tamanho}</span>
            <input type="range" min={8} max={64} value={tamanho} onChange={(e) => setTamanho(+e.target.value)} className="w-full mt-1" />
          </label>

          <div className="flex flex-col gap-3 mb-6">
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={maiusculas} onChange={() => setMaiusculas(!maiusculas)} className="w-4 h-4" />
              <span>Maiúsculas (A-Z)</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={numeros} onChange={() => setNumeros(!numeros)} className="w-4 h-4" />
              <span>Números (0-9)</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={especiais} onChange={() => setEspeciais(!especiais)} className="w-4 h-4" />
              <span>Especiais (!@#$%)</span>
            </label>
          </div>

          <button onClick={gerar} className="w-full p-3 bg-purple-600 hover:bg-purple-700 rounded-lg font-semibold transition">
            Gerar Senha
          </button>

          {resultado && (
            <div className="mt-6">
              <div className="flex items-center gap-2 bg-gray-700 p-3 rounded-lg">
                <code className="flex-1 text-sm break-all">{resultado.senha}</code>
                <button onClick={copiar} className="p-2 bg-gray-600 rounded hover:bg-gray-500" title="Copiar">
                  📋
                </button>
              </div>
              <div className="mt-3 flex items-center gap-2">
                <span className="text-sm text-gray-400">Força:</span>
                <span className={`px-2 py-1 rounded text-xs font-bold ${forcaColors[resultado.forca] || 'bg-gray-500'}`}>
                  {resultado.forca}
                </span>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
