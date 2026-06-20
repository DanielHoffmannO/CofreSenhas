import { useState } from 'react'
import { Link } from 'react-router-dom'
import api from '../services/api'
import toast from 'react-hot-toast'

export default function Settings() {
  const [step, setStep] = useState<'idle' | 'setup' | 'done'>('idle')
  const [secret, setSecret] = useState('')
  const [qrUri, setQrUri] = useState('')
  const [code, setCode] = useState('')

  const startSetup = async () => {
    const { data } = await api.post('/auth/2fa/setup')
    setSecret(data.secret)
    setQrUri(data.qrCodeUri)
    setStep('setup')
  }

  const verify = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await api.post('/auth/2fa/verify', { code })
      setStep('done')
      toast.success('2FA ativado!')
    } catch {
      toast.error('Código inválido')
    }
  }

  const disable = async () => {
    await api.post('/auth/2fa/disable')
    setStep('idle')
    toast.success('2FA desativado')
  }

  // Gerar URL do QR Code usando API pública do Google Charts
  const qrImageUrl = qrUri
    ? `https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${encodeURIComponent(qrUri)}`
    : ''

  return (
    <div className="min-h-screen bg-gray-900 p-6">
      <div className="max-w-md mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">⚙️ Configurações</h1>
          <Link to="/" className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm transition">
            ← Voltar
          </Link>
        </div>

        <div className="bg-gray-800 p-6 rounded-xl">
          <h2 className="text-lg font-semibold mb-4">🔐 Autenticação de Dois Fatores (2FA)</h2>

          {step === 'idle' && (
            <div>
              <p className="text-gray-400 text-sm mb-4">
                Adicione uma camada extra de segurança usando Google Authenticator, Authy ou similar.
              </p>
              <button onClick={startSetup} className="w-full p-3 bg-purple-600 hover:bg-purple-700 rounded-lg font-semibold transition">
                Ativar 2FA
              </button>
              <button onClick={disable} className="w-full p-3 mt-3 bg-red-600 hover:bg-red-700 rounded-lg text-sm transition">
                Desativar 2FA
              </button>
            </div>
          )}

          {step === 'setup' && (
            <div>
              <p className="text-gray-400 text-sm mb-4">
                1. Escaneie o QR Code no seu app autenticador:
              </p>
              <div className="flex justify-center mb-4">
                <img src={qrImageUrl} alt="QR Code 2FA" className="rounded-lg" />
              </div>
              <p className="text-gray-400 text-xs mb-2">Ou insira manualmente:</p>
              <code className="block bg-gray-700 p-2 rounded text-xs text-center break-all mb-4">{secret}</code>

              <p className="text-gray-400 text-sm mb-2">2. Digite o código de 6 dígitos para confirmar:</p>
              <form onSubmit={verify} className="flex gap-2">
                <input
                  type="text"
                  value={code}
                  onChange={(e) => setCode(e.target.value)}
                  placeholder="000000"
                  maxLength={6}
                  className="flex-1 p-3 bg-gray-700 rounded-lg outline-none text-center text-xl tracking-widest"
                />
                <button type="submit" className="px-4 bg-green-600 hover:bg-green-700 rounded-lg font-semibold transition">
                  ✓
                </button>
              </form>
            </div>
          )}

          {step === 'done' && (
            <div className="text-center">
              <p className="text-green-400 text-lg mb-2">✅ 2FA Ativado!</p>
              <p className="text-gray-400 text-sm">No próximo login, será pedido o código do seu autenticador.</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
