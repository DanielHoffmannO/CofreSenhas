import { useState } from 'react'
import api from '../services/api'
import toast from 'react-hot-toast'
import { copiarTexto } from '../services/clipboard'

const forcaColors: Record<string, string> = {
  Fraca: 'bg-red-500',
  Media: 'bg-yellow-500',
  Forte: 'bg-green-500',
  MuitoForte: 'bg-emerald-400',
}

type Modo = 'aleatorio' | 'palavras'

interface Props {
  /** Se informado, mostra o botão "Usar esta senha" (uso dentro do modal). */
  onUsar?: (senha: string) => void
}

export default function GeradorSenha({ onUsar }: Props) {
  const [modo, setModo] = useState<Modo>('aleatorio')

  const [tamanho, setTamanho] = useState(16)
  const [maiusculas, setMaiusculas] = useState(true)
  const [numeros, setNumeros] = useState(true)
  const [especiais, setEspeciais] = useState(true)

  const [quantidade, setQuantidade] = useState(4)
  const [capitalizar, setCapitalizar] = useState(true)
  const [incluirNumero, setIncluirNumero] = useState(true)

  const [resultado, setResultado] = useState<{ senha: string; forca: string } | null>(null)
  const [gerando, setGerando] = useState(false)

  const gerar = async () => {
    const payload =
      modo === 'aleatorio'
        ? { tipo: 'aleatorio', tamanho, usarMaiusculas: maiusculas, usarNumeros: numeros, usarEspeciais: especiais }
        : { tipo: 'palavras', quantidade, capitalizar, incluirNumero }
    setGerando(true)
    try {
      const { data } = await api.post('/gerador', payload)
      setResultado(data)
    } catch {
      toast.error('Não foi possível gerar a senha')
    } finally {
      setGerando(false)
    }
  }

  const abaClasse = (ativa: boolean) =>
    `flex-1 p-2 rounded-lg text-sm font-semibold transition ${ativa ? 'bg-purple-600' : 'bg-gray-700 hover:bg-gray-600'}`

  return (
    <div>
      <div className="flex gap-2 mb-6">
        <button type="button" onClick={() => setModo('aleatorio')} className={abaClasse(modo === 'aleatorio')}>
          Aleatória
        </button>
        <button type="button" onClick={() => setModo('palavras')} className={abaClasse(modo === 'palavras')}>
          Frase (PT)
        </button>
      </div>

      {modo === 'aleatorio' ? (
        <>
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
        </>
      ) : (
        <>
          <label className="block mb-4">
            <span className="text-sm text-gray-400">Palavras: {quantidade}</span>
            <input type="range" min={3} max={8} value={quantidade} onChange={(e) => setQuantidade(+e.target.value)} className="w-full mt-1" />
          </label>
          <div className="flex flex-col gap-3 mb-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={capitalizar} onChange={() => setCapitalizar(!capitalizar)} className="w-4 h-4" />
              <span>Capitalizar (Cavalo em vez de cavalo)</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={incluirNumero} onChange={() => setIncluirNumero(!incluirNumero)} className="w-4 h-4" />
              <span>Incluir número no final</span>
            </label>
          </div>
          <p className="text-xs text-gray-500 mb-6">
            Palavras comuns em português (sem acentos), separadas por hífen: mais fáceis de decorar e digitar.
          </p>
        </>
      )}

      <button type="button" onClick={gerar} disabled={gerando} className="w-full p-3 bg-purple-600 hover:bg-purple-700 disabled:opacity-60 rounded-lg font-semibold transition">
        {gerando ? 'Gerando...' : 'Gerar Senha'}
      </button>

      {resultado && (
        <div className="mt-6">
          <div className="flex items-center gap-2 bg-gray-700 p-3 rounded-lg">
            <code className="flex-1 text-sm break-all">{resultado.senha}</code>
            <button type="button" onClick={() => copiarTexto(resultado.senha)} className="p-2 bg-gray-600 rounded hover:bg-gray-500" title="Copiar" aria-label="Copiar senha gerada">
              📋
            </button>
          </div>
          <div className="mt-3 flex items-center gap-2">
            <span className="text-sm text-gray-400">Força:</span>
            <span className={`px-2 py-1 rounded text-xs font-bold ${forcaColors[resultado.forca] || 'bg-gray-500'}`}>{resultado.forca}</span>
          </div>
          {onUsar && (
            <button type="button" onClick={() => onUsar(resultado.senha)} className="w-full mt-4 p-3 bg-green-600 hover:bg-green-700 rounded-lg font-semibold transition">
              Usar esta senha
            </button>
          )}
        </div>
      )}
    </div>
  )
}
