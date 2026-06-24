using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Security.Cryptography;
using System.Text;
using CofreSenhas.Domain.DTOs.Auth;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.Extensions.Configuration;
using Microsoft.IdentityModel.Tokens;
using OtpNet;

namespace CofreSenhas.Service.Services;

public class AuthService : IAuthService
{
    private readonly IUsuarioRepository _usuarioRepository;
    private readonly IConfiguration _configuration;

    public AuthService(IUsuarioRepository usuarioRepository, IConfiguration configuration)
    {
        _usuarioRepository = usuarioRepository;
        _configuration = configuration;
    }

    public async Task<AuthResponse> RegistrarAsync(RegisterRequest request)
    {
        var existente = await _usuarioRepository.GetByEmailAsync(request.Email);
        if (existente is not null)
            throw new InvalidOperationException("Email já cadastrado.");

        var usuario = new Usuario
        {
            Nome = request.Nome,
            Email = request.Email,
            SenhaHash = BCrypt.Net.BCrypt.HashPassword(request.Senha),
            CriadoEm = DateTime.UtcNow
        };

        await _usuarioRepository.AddAsync(usuario);
        await _usuarioRepository.SaveChangesAsync();

        return new AuthResponse(GerarToken(usuario));
    }

    public async Task<AuthResponse> LoginAsync(LoginRequest request)
    {
        var usuario = await _usuarioRepository.GetByEmailAsync(request.Email)
            ?? throw new UnauthorizedAccessException("Credenciais inválidas.");

        if (!BCrypt.Net.BCrypt.Verify(request.Senha, usuario.SenhaHash))
            throw new UnauthorizedAccessException("Credenciais inválidas.");

        if (usuario.TwoFactorEnabled)
        {
            if (string.IsNullOrEmpty(request.TotpCode))
                throw new InvalidOperationException("2FA_REQUIRED");

            var totp = new Totp(Base32Encoding.ToBytes(usuario.TwoFactorSecret!));
            if (!totp.VerifyTotp(request.TotpCode, out _, new VerificationWindow(1, 1)))
                throw new UnauthorizedAccessException("Código 2FA inválido.");
        }

        return new AuthResponse(GerarToken(usuario));
    }

    public async Task<Setup2faResponse> Setup2faAsync(int userId)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        var secret = Base32Encoding.ToString(KeyGeneration.GenerateRandomKey(20));
        usuario.TwoFactorSecret = secret;
        await _usuarioRepository.SaveChangesAsync();

        var uri = $"otpauth://totp/CofreSenhas:{usuario.Email}?secret={secret}&issuer=CofreSenhas&digits=6";
        return new Setup2faResponse(secret, uri);
    }

    public async Task<bool> Verify2faAsync(int userId, string code)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        if (string.IsNullOrEmpty(usuario.TwoFactorSecret))
            throw new InvalidOperationException("2FA não configurado.");

        var totp = new Totp(Base32Encoding.ToBytes(usuario.TwoFactorSecret));
        if (!totp.VerifyTotp(code, out _, new VerificationWindow(1, 1)))
            return false;

        usuario.TwoFactorEnabled = true;
        await _usuarioRepository.SaveChangesAsync();
        return true;
    }

    public async Task Disable2faAsync(int userId)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        usuario.TwoFactorEnabled = false;
        usuario.TwoFactorSecret = null;
        await _usuarioRepository.SaveChangesAsync();
    }

    public async Task SetupMasterPasswordAsync(int userId, string masterPassword)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        var salt = RandomNumberGenerator.GetBytes(32);
        usuario.MasterPasswordSalt = Convert.ToBase64String(salt);
        usuario.MasterPasswordHash = HashMasterPassword(masterPassword, salt);
        await _usuarioRepository.SaveChangesAsync();
    }

    public async Task<bool> VerifyMasterPasswordAsync(int userId, string masterPassword)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        if (usuario.MasterPasswordSalt is null || usuario.MasterPasswordHash is null)
            return false;

        var salt = Convert.FromBase64String(usuario.MasterPasswordSalt);
        var hash = HashMasterPassword(masterPassword, salt);
        return hash == usuario.MasterPasswordHash;
    }

    public async Task<MasterPasswordStatusResponse> GetMasterPasswordStatusAsync(int userId)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        return new MasterPasswordStatusResponse(usuario.MasterPasswordHash is not null);
    }

    public async Task<ProfileResponse> GetProfileAsync(int userId)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        return new ProfileResponse(
            usuario.Id, usuario.Nome, usuario.Email, usuario.CriadoEm,
            usuario.TwoFactorEnabled, usuario.MasterPasswordHash is not null);
    }

    public async Task ChangePasswordAsync(int userId, ChangePasswordRequest request)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId)
            ?? throw new InvalidOperationException("Usuário não encontrado.");

        if (!BCrypt.Net.BCrypt.Verify(request.SenhaAtual, usuario.SenhaHash))
            throw new UnauthorizedAccessException("Senha atual incorreta.");

        usuario.SenhaHash = BCrypt.Net.BCrypt.HashPassword(request.NovaSenha);
        await _usuarioRepository.SaveChangesAsync();
    }

    private static string HashMasterPassword(string password, byte[] salt)
    {
        using var pbkdf2 = new Rfc2898DeriveBytes(
            Encoding.UTF8.GetBytes(password), salt, 100_000, HashAlgorithmName.SHA256);
        return Convert.ToBase64String(pbkdf2.GetBytes(32));
    }

    private string GerarToken(Usuario usuario)
    {
        var key = new SymmetricSecurityKey(Encoding.UTF8.GetBytes(_configuration["Jwt:Key"]!));
        var credentials = new SigningCredentials(key, SecurityAlgorithms.HmacSha256);

        var claims = new[]
        {
            new Claim(ClaimTypes.NameIdentifier, usuario.Id.ToString()),
            new Claim(ClaimTypes.Email, usuario.Email),
            new Claim(ClaimTypes.Name, usuario.Nome)
        };

        var token = new JwtSecurityToken(
            issuer: _configuration["Jwt:Issuer"],
            audience: _configuration["Jwt:Audience"],
            claims: claims,
            expires: DateTime.UtcNow.AddHours(8),
            signingCredentials: credentials);

        return new JwtSecurityTokenHandler().WriteToken(token);
    }
}
