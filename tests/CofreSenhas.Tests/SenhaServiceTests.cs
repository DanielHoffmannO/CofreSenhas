using CofreSenhas.Domain.DTOs.Senhas;
using CofreSenhas.Persistence.Data;
using CofreSenhas.Persistence.Repositories;
using CofreSenhas.Service.Services;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;

namespace CofreSenhas.Tests;

public class SenhaServiceTests : IDisposable
{
    private readonly AppDbContext _context;
    private readonly SenhaService _service;

    public SenhaServiceTests()
    {
        var options = new DbContextOptionsBuilder<AppDbContext>()
            .UseInMemoryDatabase(Guid.NewGuid().ToString())
            .Options;

        _context = new AppDbContext(options);

        var config = new ConfigurationBuilder()
            .AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["Encryption:Key"] = "CofreSenhas-AES-256-Key-32Chars!"
            })
            .Build();

        var repo = new SenhaRepository(_context);
        var versaoRepo = new SenhaVersaoRepository(_context);
        var usuarioRepo = new UsuarioRepository(_context);
        _service = new SenhaService(repo, versaoRepo, usuarioRepo, config);
    }

    [Fact]
    public async Task Criar_SalvaCorretamente()
    {
        var request = new CriarSenhaRequest("Gmail", "user@gmail.com", "pass123", "https://gmail.com", null);

        var result = await _service.CriarAsync(request, 1);

        Assert.Equal("Gmail", result.Titulo);
        Assert.Equal("user@gmail.com", result.Login);
        Assert.Equal("pass123", result.Senha);
    }

    [Fact]
    public async Task GetByUsuario_RetornaSoDoUser()
    {
        await _service.CriarAsync(new CriarSenhaRequest("User1", "a", "p", null, null), 1);
        await _service.CriarAsync(new CriarSenhaRequest("User2", "b", "p", null, null), 2);
        await _service.CriarAsync(new CriarSenhaRequest("User1b", "c", "p", null, null), 1);

        var result = (await _service.GetByUsuarioAsync(1)).ToList();

        Assert.Equal(2, result.Count);
        Assert.All(result, r => Assert.StartsWith("User1", r.Titulo));
    }

    public void Dispose() => _context.Dispose();
}
