using CofreSenhas.Domain.DTOs.Gerador;
using CofreSenhas.Domain.Enums;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface IGeradorSenhaService
{
    Task<GerarSenhaResponse> GerarAsync(GerarSenhaRequest request, int userId);
    ForcaSenha CalcularForca(string senha);
}
