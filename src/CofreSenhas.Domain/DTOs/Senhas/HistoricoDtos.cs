namespace CofreSenhas.Domain.DTOs.Senhas;

public record SenhaVersaoResponse(int Id, string Titulo, string Login, string Senha, DateTime AlteradoEm);
public record RestaurarVersaoRequest(string MasterPassword);
