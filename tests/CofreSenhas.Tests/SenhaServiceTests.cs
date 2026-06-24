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

    [Fact]
    public async Task GetByUsuarioPaged_RetornaPaginaCorreta()
    {
        for (int i = 1; i <= 15; i++)
            await _service.CriarAsync(new CriarSenhaRequest($"Senha{i}", "login", "pass", null, null), 1);

        var page1 = await _service.GetByUsuarioPagedAsync(1, 1, 10);
        var page2 = await _service.GetByUsuarioPagedAsync(1, 2, 10);

        Assert.Equal(10, page1.Items.Count());
        Assert.Equal(5, page2.Items.Count());
        Assert.Equal(15, page1.TotalCount);
        Assert.Equal(2, page1.TotalPages);
    }

    [Fact]
    public async Task GetById_OutroUsuario_RetornaNull()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Minha", "a", "p", null, null), 1);

        var result = await _service.GetByIdAsync(criada.Id, 999);

        Assert.Null(result);
    }

    [Fact]
    public async Task Atualizar_SalvaVersaoAnterior()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("V1", "login1", "pass1", null, null), 1);

        await _service.AtualizarAsync(criada.Id, new CriarSenhaRequest("V2", "login2", "pass2", null, null), 1);

        var historico = (await _service.GetHistoricoAsync(criada.Id, 1)).ToList();
        Assert.Single(historico);
        Assert.Equal("V1", historico[0].Titulo);
        Assert.Equal("pass1", historico[0].Senha);
    }

    [Fact]
    public async Task Atualizar_OutroUsuario_RetornaNull()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Test", "a", "p", null, null), 1);

        var result = await _service.AtualizarAsync(criada.Id, new CriarSenhaRequest("Hack", "b", "x", null, null), 999);

        Assert.Null(result);
    }

    [Fact]
    public async Task Deletar_RetornaTrue()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Del", "a", "p", null, null), 1);

        var result = await _service.DeletarAsync(criada.Id, 1);

        Assert.True(result);
        Assert.Null(await _service.GetByIdAsync(criada.Id, 1));
    }

    [Fact]
    public async Task Deletar_OutroUsuario_RetornaFalse()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Del", "a", "p", null, null), 1);

        var result = await _service.DeletarAsync(criada.Id, 999);

        Assert.False(result);
    }

    [Fact]
    public async Task RestaurarVersao_RestauraDadosAntigos()
    {
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Original", "login1", "pass1", null, null), 1);
        await _service.AtualizarAsync(criada.Id, new CriarSenhaRequest("Alterada", "login2", "pass2", null, null), 1);

        var historico = (await _service.GetHistoricoAsync(criada.Id, 1)).ToList();
        var restaurada = await _service.RestaurarVersaoAsync(criada.Id, historico[0].Id, 1);

        Assert.NotNull(restaurada);
        Assert.Equal("Original", restaurada!.Titulo);
        Assert.Equal("pass1", restaurada.Senha);
    }

    [Fact]
    public async Task Criar_ComCategoria_SalvaCategoria()
    {
        var result = await _service.CriarAsync(new CriarSenhaRequest("Banco", "login", "pass", null, null, "Banco"), 1);

        Assert.Equal("Banco", result.Categoria);
    }

    [Fact]
    public async Task CriptografiaFunciona_SenhaDecriptografadaIgualOriginal()
    {
        var original = "MinhaSenha$uper$ecreta!@#123";
        var criada = await _service.CriarAsync(new CriarSenhaRequest("Test", "user", original, null, null), 1);

        var recuperada = await _service.GetByIdAsync(criada.Id, 1);

        Assert.Equal(original, recuperada!.Senha);
    }

    public void Dispose() => _context.Dispose();
}
