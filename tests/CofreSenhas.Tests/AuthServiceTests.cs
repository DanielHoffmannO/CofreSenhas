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

    [Fact]
    public async Task Login_EmailInexistente_LancaExcecao()
    {
        await Assert.ThrowsAsync<UnauthorizedAccessException>(
            () => _service.LoginAsync(new LoginRequest("naoexiste@email.com", "qualquer")));
    }

    [Fact]
    public async Task GetProfile_RetornaDadosCorretos()
    {
        await _service.RegistrarAsync(new RegisterRequest("Daniel", "daniel@email.com", "123456"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "daniel@email.com");

        var profile = await _service.GetProfileAsync(usuario.Id);

        Assert.Equal("Daniel", profile.Nome);
        Assert.Equal("daniel@email.com", profile.Email);
        Assert.False(profile.TwoFactorEnabled);
        Assert.False(profile.MasterPasswordConfigured);
    }

    [Fact]
    public async Task ChangePassword_SenhaAtualCorreta_Altera()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "change@email.com", "senhaAntiga"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "change@email.com");

        await _service.ChangePasswordAsync(usuario.Id, new ChangePasswordRequest("senhaAntiga", "senhaNova"));

        // Login com nova senha deve funcionar
        var result = await _service.LoginAsync(new LoginRequest("change@email.com", "senhaNova"));
        Assert.NotNull(result.Token);
    }

    [Fact]
    public async Task ChangePassword_SenhaAtualErrada_LancaExcecao()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "changefail@email.com", "senhaCorreta"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "changefail@email.com");

        await Assert.ThrowsAsync<UnauthorizedAccessException>(
            () => _service.ChangePasswordAsync(usuario.Id, new ChangePasswordRequest("senhaErrada", "novaSenha")));
    }

    [Fact]
    public async Task SetupMasterPassword_ConfiguraCorretamente()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "master@email.com", "123456"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "master@email.com");

        await _service.SetupMasterPasswordAsync(usuario.Id, "minha-master-password");

        var status = await _service.GetMasterPasswordStatusAsync(usuario.Id);
        Assert.True(status.IsConfigured);
    }

    [Fact]
    public async Task VerifyMasterPassword_Correta_RetornaTrue()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "verify@email.com", "123456"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "verify@email.com");
        await _service.SetupMasterPasswordAsync(usuario.Id, "master123");

        var result = await _service.VerifyMasterPasswordAsync(usuario.Id, "master123");

        Assert.True(result);
    }

    [Fact]
    public async Task VerifyMasterPassword_Errada_RetornaFalse()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "verifyfail@email.com", "123456"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "verifyfail@email.com");
        await _service.SetupMasterPasswordAsync(usuario.Id, "master123");

        var result = await _service.VerifyMasterPasswordAsync(usuario.Id, "errada");

        Assert.False(result);
    }

    [Fact]
    public async Task Setup2fa_RetornaSecretEUri()
    {
        await _service.RegistrarAsync(new RegisterRequest("Test", "2fa@email.com", "123456"));
        var usuario = await _context.Usuarios.FirstAsync(u => u.Email == "2fa@email.com");

        var result = await _service.Setup2faAsync(usuario.Id);

        Assert.False(string.IsNullOrEmpty(result.Secret));
        Assert.Contains("otpauth://totp/", result.QrCodeUri);
        Assert.Contains("2fa@email.com", result.QrCodeUri);
    }

    public void Dispose() => _context.Dispose();
}
