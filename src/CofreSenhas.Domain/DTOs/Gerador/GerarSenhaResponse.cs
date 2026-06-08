using CofreSenhas.Domain.Enums;

namespace CofreSenhas.Domain.DTOs.Gerador;

public record GerarSenhaResponse(string Senha, ForcaSenha Forca);
