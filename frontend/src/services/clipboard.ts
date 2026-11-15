import toast from 'react-hot-toast'

// `navigator.clipboard` só existe em contexto seguro (HTTPS ou localhost).
// O cofre roda em HTTP na LAN, então cai para o `execCommand` antigo e só
// anuncia "Copiado!" se a cópia realmente funcionou.
const LIMPAR_APOS_MS = 30_000

// Apaga a área de transferência depois de um tempo, mas só se ela ainda
// guardar o que copiamos (não atropela uma cópia nova do usuário).
function agendarLimpeza(texto: string) {
  window.setTimeout(async () => {
    try {
      if ((await navigator.clipboard.readText()) === texto) await navigator.clipboard.writeText('')
    } catch {
      // sem permissão de leitura (ou HTTP): não há como limpar com segurança
    }
  }, LIMPAR_APOS_MS)
}

export async function copiarTexto(texto: string): Promise<void> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(texto)
    } else {
      const area = document.createElement('textarea')
      area.value = texto
      area.setAttribute('readonly', '')
      area.style.position = 'fixed'
      area.style.opacity = '0'
      document.body.appendChild(area)
      area.select()
      const ok = document.execCommand('copy')
      document.body.removeChild(area)
      if (!ok) throw new Error('execCommand falhou')
    }
    toast.success('Copiado! (limpa em 30s)')
    agendarLimpeza(texto)
  } catch {
    toast.error('Não foi possível copiar')
  }
}
