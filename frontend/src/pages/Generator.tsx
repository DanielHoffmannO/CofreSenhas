import { Link } from 'react-router-dom'
import PageTransition from '../components/PageTransition'
import GeradorSenha from '../components/GeradorSenha'

export default function Generator() {
  return (
    <PageTransition>
    <div className="min-h-screen bg-gray-900 p-6">
      <div className="max-w-md mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">⚡ Gerador de Senhas</h1>
          <Link to="/" className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm transition">
            ← Voltar
          </Link>
        </div>

        <div className="bg-gray-800 p-6 rounded-xl">
          <GeradorSenha />
        </div>
      </div>
    </div>
    </PageTransition>
  )
}
