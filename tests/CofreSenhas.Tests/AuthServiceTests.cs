using CofreSenhas.Domain.DTOs.Auth;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Persistence.Data;
using CofreSenhas.Persistence.Repositories;
using CofreSenhas.Service.Services;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;

namespace CofreSenhas.Tests;

public class AuthServiceTests : IDisposable
{
    private readonly AppDbContext _context;
    private readonly AuthService _service;

    public AuthServiceTests()
    {
        var options = new DbContextOptionsBuilder<AppDbContext>()
            .UseInMemoryDatabase(Guid.NewGuid().ToString())
            .Options;

        _context = new AppDbContext(options);

        var config = new ConfigurationBuilder()
            .AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["Jwt:Key"] = "CofreSenhas-SuperSecret-Key-2024-Portfolio-MinLength32!!",
                ["Jwt:Issuer"] = "CofreSenhas",
                ["Jwt:Audience"] = "CofreSenhas"
            })
            .Build();

        var usuarioRepo = new UsuarioRepository(_context);
        _service = new AuthService(usuarioRepo, config);
    }

    [Fact]
    public async Task Registrar_RetornaToken()
    {
        var request = new RegisterRequest("Test", "test@email.com", "senha123");

        var result = await _service.RegistrarAsync(request);

        Assert.NotNull(result);
        Assert.False(string.IsNullOrEmpty(result.Token));
    }

    [Fact]
    public async Task Registrar_EmailDuplicado_LancaExcecao()
    {
        var request = new RegisterRequest("Test", "dup@email.com", "senha123");
        await _service.RegistrarAsync(request);

        await Assert.ThrowsAsync<InvalidOperationException>(
            () => _service.RegistrarAsync(request));
    }

    [Fact]
    public async Task Login_Correto_RetornaToken()
    {
        var register = new RegisterRequest("Test", "login@email.com", "senha123");
        await _service.RegistrarAsync(register);

        var result = await _service.LoginAsync(new LoginRequest("login@email.com", "senha123"));

        Assert.NotNull(result);
        Assert.False(string.IsNullOrEmpty(result.Token));
    }

    [Fact]
    public async Task Login_SenhaErrada_LancaExcecao()
    {
        var register = new RegisterRequest("Test", "wrong@email.com", "senha123");
        await _service.RegistrarAsync(register);

        await Assert.ThrowsAsync<UnauthorizedAccessException>(
            () => _service.LoginAsync(new LoginRequest("wrong@email.com", "errada")));
    }

    public void Dispose() => _context.Dispose();
}
