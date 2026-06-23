namespace CofreSenhas.Domain.Entities;

public class SenhaVersao
{
    public int Id { get; set; }
    public int SenhaId { get; set; }
    public string SenhaCriptografada { get; set; } = string.Empty;
    public string Titulo { get; set; } = string.Empty;
    public string Login { get; set; } = string.Empty;
    public DateTime AlteradoEm { get; set; }
    public Senha Senha { get; set; } = null!;
}
