using CofreSenhas.Domain.DTOs.Gerador;
using CofreSenhas.Domain.Enums;
using CofreSenhas.Persistence.Data;
using CofreSenhas.Persistence.Repositories;
using CofreSenhas.Service.Services;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Tests;

public class GeradorSenhaServiceTests : IDisposable
{
    private readonly AppDbContext _context;
    private readonly GeradorSenhaService _service;

    public GeradorSenhaServiceTests()
    {
        var options = new DbContextOptionsBuilder<AppDbContext>()
            .UseInMemoryDatabase(Guid.NewGuid().ToString())
            .Options;

        _context = new AppDbContext(options);
        var repo = new HistoricoGeracaoRepository(_context);
        _service = new GeradorSenhaService(repo);
    }

    [Fact]
    public async Task Gerar_Com8Chars_Retorna8Chars()
    {
        var request = new GerarSenhaRequest(8, true, true, false);

        var result = await _service.GerarAsync(request, 1);

        Assert.Equal(8, result.Senha.Length);
    }

    [Fact]
    public async Task Gerar_ComEspeciais_ContemEspeciais()
    {
        var request = new GerarSenhaRequest(20, true, true, true);

        var result = await _service.GerarAsync(request, 1);

        Assert.Contains(result.Senha, c => "!@#$%^&*()_+-=[]{}|;:,.<>?".Contains(c));
    }

    [Fact]
    public void CalcularForca_RetornaCorreto()
    {
        Assert.Equal(ForcaSenha.Fraca, _service.CalcularForca("abc"));              // curta, só lower = 0 pts
        Assert.Equal(ForcaSenha.Fraca, _service.CalcularForca("abcdefgh"));         // >=8 = 1 pt
        Assert.Equal(ForcaSenha.Media, _service.CalcularForca("abcdefgh1"));        // >=8 + digit = 2 pts
        Assert.Equal(ForcaSenha.Forte, _service.CalcularForca("Abcdefgh1"));        // >=8 + upper + digit = 3 pts
        Assert.Equal(ForcaSenha.MuitoForte, _service.CalcularForca("Abcdefghijk1!")); // >=8 + >=12 + upper + digit + special = 5 pts
    }

    public void Dispose() => _context.Dispose();
}
