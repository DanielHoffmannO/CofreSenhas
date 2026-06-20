namespace CofreSenhas.Domain.Entities;

public class AuditLog
{
    public int Id { get; set; }
    public int UsuarioId { get; set; }
    public int SenhaId { get; set; }
    public string Acao { get; set; } = string.Empty; // "Visualizada", "Copiada", "Criada", "Editada", "Deletada"
    public DateTime DataHora { get; set; }
}
