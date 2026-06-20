namespace CofreSenhas.Domain.DTOs.Senhas;

public record CriarSenhaRequest(string Titulo, string Login, string Senha, string? Url, string? Notas, string Categoria = "Pessoal");
