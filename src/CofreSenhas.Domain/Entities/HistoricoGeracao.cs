using CofreSenhas.Domain.Enums;

namespace CofreSenhas.Domain.Entities;

public class HistoricoGeracao
{
    public int Id { get; set; }
    public int UsuarioId { get; set; }
    public string Prompt { get; set; } = string.Empty;
    public string SenhaGerada { get; set; } = string.Empty;
    public ForcaSenha ForcaSenha { get; set; }
    public DateTime CriadoEm { get; set; }
    public Usuario Usuario { get; set; } = null!;
}
