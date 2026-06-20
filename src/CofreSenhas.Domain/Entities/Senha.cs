namespace CofreSenhas.Domain.Entities;

public class Senha
{
    public int Id { get; set; }
    public int UsuarioId { get; set; }
    public string Titulo { get; set; } = string.Empty;
    public string Login { get; set; } = string.Empty;
    public string SenhaCriptografada { get; set; } = string.Empty;
    public string? Url { get; set; }
    public string? Notas { get; set; }
    public string Categoria { get; set; } = "Pessoal";
    public DateTime CriadoEm { get; set; }
    public DateTime AtualizadoEm { get; set; }
    public Usuario Usuario { get; set; } = null!;
}
