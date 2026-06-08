using System.Security.Cryptography;
using CofreSenhas.Domain.DTOs.Gerador;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Enums;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Domain.Interfaces.Services;

namespace CofreSenhas.Service.Services;

public class GeradorSenhaService : IGeradorSenhaService
{
    private readonly IHistoricoGeracaoRepository _historicoRepository;

    public GeradorSenhaService(IHistoricoGeracaoRepository historicoRepository)
    {
        _historicoRepository = historicoRepository;
    }

    public async Task<GerarSenhaResponse> GerarAsync(GerarSenhaRequest request, int userId)
    {
        var chars = "abcdefghijklmnopqrstuvwxyz";
        if (request.UsarMaiusculas) chars += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        if (request.UsarNumeros) chars += "0123456789";
        if (request.UsarEspeciais) chars += "!@#$%^&*()_+-=[]{}|;:,.<>?";

        var senha = new string(Enumerable.Range(0, request.Tamanho)
            .Select(_ => chars[RandomNumberGenerator.GetInt32(chars.Length)])
            .ToArray());

        var forca = CalcularForca(senha);

        var historico = new HistoricoGeracao
        {
            UsuarioId = userId,
            Prompt = $"Tamanho={request.Tamanho}, Maiúsculas={request.UsarMaiusculas}, Números={request.UsarNumeros}, Especiais={request.UsarEspeciais}",
            SenhaGerada = senha,
            ForcaSenha = forca,
            CriadoEm = DateTime.UtcNow
        };

        await _historicoRepository.AddAsync(historico);
        await _historicoRepository.SaveChangesAsync();

        return new GerarSenhaResponse(senha, forca);
    }

    public ForcaSenha CalcularForca(string senha) => senha.Length switch
    {
        < 8 => ForcaSenha.Fraca,
        < 12 => ForcaSenha.Media,
        < 16 => ForcaSenha.Forte,
        _ => ForcaSenha.MuitoForte
    };
}
