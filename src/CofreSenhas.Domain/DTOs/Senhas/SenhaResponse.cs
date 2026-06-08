namespace CofreSenhas.Domain.DTOs.Senhas;

public record SenhaResponse(int Id, string Titulo, string Login, string Senha, string? Url, string? Notas, DateTime CriadoEm);
