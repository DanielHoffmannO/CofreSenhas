using System.Security.Cryptography;
using System.Text;
using CofreSenhas.Domain.DTOs.Senhas;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.Extensions.Configuration;

namespace CofreSenhas.Service.Services;

public class SenhaService : ISenhaService
{
    private readonly ISenhaRepository _senhaRepository;
    private readonly byte[] _chave;

    public SenhaService(ISenhaRepository senhaRepository, IConfiguration configuration)
    {
        _senhaRepository = senhaRepository;
        _chave = Encoding.UTF8.GetBytes(configuration["Encryption:Key"]![..32]);
    }

    public async Task<IEnumerable<SenhaResponse>> GetByUsuarioAsync(int userId)
    {
        var senhas = await _senhaRepository.GetByUsuarioIdAsync(userId);
        return senhas.Select(s => ToResponse(s));
    }

    public async Task<SenhaResponse?> GetByIdAsync(int id, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return null;
        return ToResponse(senha);
    }

    public async Task<SenhaResponse> CriarAsync(CriarSenhaRequest request, int userId)
    {
        var senha = new Senha
        {
            UsuarioId = userId,
            Titulo = request.Titulo,
            Login = request.Login,
            SenhaCriptografada = Encrypt(request.Senha),
            Url = request.Url,
            Notas = request.Notas,
            Categoria = request.Categoria,
            CriadoEm = DateTime.UtcNow,
            AtualizadoEm = DateTime.UtcNow
        };

        await _senhaRepository.AddAsync(senha);
        await _senhaRepository.SaveChangesAsync();
        return ToResponse(senha);
    }

    public async Task<SenhaResponse?> AtualizarAsync(int id, CriarSenhaRequest request, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return null;

        senha.Titulo = request.Titulo;
        senha.Login = request.Login;
        senha.SenhaCriptografada = Encrypt(request.Senha);
        senha.Url = request.Url;
        senha.Notas = request.Notas;
        senha.Categoria = request.Categoria;
        senha.AtualizadoEm = DateTime.UtcNow;

        _senhaRepository.Update(senha);
        await _senhaRepository.SaveChangesAsync();
        return ToResponse(senha);
    }

    public async Task<bool> DeletarAsync(int id, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return false;

        _senhaRepository.Delete(senha);
        await _senhaRepository.SaveChangesAsync();
        return true;
    }

    private SenhaResponse ToResponse(Senha s) =>
        new(s.Id, s.Titulo, s.Login, Decrypt(s.SenhaCriptografada), s.Url, s.Notas, s.Categoria, s.CriadoEm);

    private string Encrypt(string texto)
    {
        using var aes = Aes.Create();
        aes.Key = _chave;
        aes.GenerateIV();

        using var encryptor = aes.CreateEncryptor();
        var bytes = Encoding.UTF8.GetBytes(texto);
        var encrypted = encryptor.TransformFinalBlock(bytes, 0, bytes.Length);

        var result = new byte[aes.IV.Length + encrypted.Length];
        aes.IV.CopyTo(result, 0);
        encrypted.CopyTo(result, aes.IV.Length);
        return Convert.ToBase64String(result);
    }

    private string Decrypt(string cifrado)
    {
        var data = Convert.FromBase64String(cifrado);
        using var aes = Aes.Create();
        aes.Key = _chave;

        var iv = data[..16];
        var cipher = data[16..];
        aes.IV = iv;

        using var decryptor = aes.CreateDecryptor();
        var decrypted = decryptor.TransformFinalBlock(cipher, 0, cipher.Length);
        return Encoding.UTF8.GetString(decrypted);
    }
}
