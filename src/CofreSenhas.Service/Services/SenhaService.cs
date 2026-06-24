using System.Security.Cryptography;
using System.Text;
using CofreSenhas.Domain.DTOs;
using CofreSenhas.Domain.DTOs.Senhas;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.Extensions.Configuration;

namespace CofreSenhas.Service.Services;

public class SenhaService : ISenhaService
{
    private readonly ISenhaRepository _senhaRepository;
    private readonly ISenhaVersaoRepository _versaoRepository;
    private readonly IUsuarioRepository _usuarioRepository;
    private readonly byte[] _chaveGlobal;

    public SenhaService(
        ISenhaRepository senhaRepository,
        ISenhaVersaoRepository versaoRepository,
        IUsuarioRepository usuarioRepository,
        IConfiguration configuration)
    {
        _senhaRepository = senhaRepository;
        _versaoRepository = versaoRepository;
        _usuarioRepository = usuarioRepository;
        _chaveGlobal = Encoding.UTF8.GetBytes(configuration["Encryption:Key"]![..32]);
    }

    public async Task<IEnumerable<SenhaResponse>> GetByUsuarioAsync(int userId)
    {
        var senhas = await _senhaRepository.GetByUsuarioIdAsync(userId);
        var chave = await ObterChaveAsync(userId);
        return senhas.Select(s => ToResponse(s, chave));
    }

    public async Task<PagedResponse<SenhaResponse>> GetByUsuarioPagedAsync(int userId, int page, int pageSize)
    {
        var (senhas, total) = await _senhaRepository.GetByUsuarioIdPagedAsync(userId, page, pageSize);
        var chave = await ObterChaveAsync(userId);
        var items = senhas.Select(s => ToResponse(s, chave));
        return new PagedResponse<SenhaResponse>(items, page, pageSize, total);
    }

    public async Task<SenhaResponse?> GetByIdAsync(int id, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return null;
        var chave = await ObterChaveAsync(userId);
        return ToResponse(senha, chave);
    }

    public async Task<SenhaResponse> CriarAsync(CriarSenhaRequest request, int userId)
    {
        var chave = await ObterChaveAsync(userId);
        var senha = new Senha
        {
            UsuarioId = userId,
            Titulo = request.Titulo,
            Login = request.Login,
            SenhaCriptografada = Encrypt(request.Senha, chave),
            Url = request.Url,
            Notas = request.Notas,
            Categoria = request.Categoria,
            CriadoEm = DateTime.UtcNow,
            AtualizadoEm = DateTime.UtcNow
        };

        await _senhaRepository.AddAsync(senha);
        await _senhaRepository.SaveChangesAsync();
        return ToResponse(senha, chave);
    }

    public async Task<SenhaResponse?> AtualizarAsync(int id, CriarSenhaRequest request, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return null;

        var chave = await ObterChaveAsync(userId);

        // Salvar versão anterior no histórico
        var versao = new SenhaVersao
        {
            SenhaId = senha.Id,
            Titulo = senha.Titulo,
            Login = senha.Login,
            SenhaCriptografada = senha.SenhaCriptografada,
            AlteradoEm = senha.AtualizadoEm
        };
        await _versaoRepository.AddAsync(versao);

        senha.Titulo = request.Titulo;
        senha.Login = request.Login;
        senha.SenhaCriptografada = Encrypt(request.Senha, chave);
        senha.Url = request.Url;
        senha.Notas = request.Notas;
        senha.Categoria = request.Categoria;
        senha.AtualizadoEm = DateTime.UtcNow;

        _senhaRepository.Update(senha);
        await _senhaRepository.SaveChangesAsync();
        return ToResponse(senha, chave);
    }

    public async Task<bool> DeletarAsync(int id, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(id);
        if (senha is null || senha.UsuarioId != userId) return false;

        _senhaRepository.Delete(senha);
        await _senhaRepository.SaveChangesAsync();
        return true;
    }

    public async Task<IEnumerable<SenhaVersaoResponse>> GetHistoricoAsync(int senhaId, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(senhaId);
        if (senha is null || senha.UsuarioId != userId) return [];

        var chave = await ObterChaveAsync(userId);
        var versoes = await _versaoRepository.GetBySenhaIdAsync(senhaId);
        return versoes.Select(v => new SenhaVersaoResponse(
            v.Id, v.Titulo, v.Login, Decrypt(v.SenhaCriptografada, chave), v.AlteradoEm));
    }

    public async Task<SenhaResponse?> RestaurarVersaoAsync(int senhaId, int versaoId, int userId)
    {
        var senha = await _senhaRepository.GetByIdAsync(senhaId);
        if (senha is null || senha.UsuarioId != userId) return null;

        var versao = await _versaoRepository.GetByIdAsync(versaoId);
        if (versao is null || versao.SenhaId != senhaId) return null;

        // Salvar estado atual como versão antes de restaurar
        var versaoAtual = new SenhaVersao
        {
            SenhaId = senha.Id,
            Titulo = senha.Titulo,
            Login = senha.Login,
            SenhaCriptografada = senha.SenhaCriptografada,
            AlteradoEm = senha.AtualizadoEm
        };
        await _versaoRepository.AddAsync(versaoAtual);

        senha.Titulo = versao.Titulo;
        senha.Login = versao.Login;
        senha.SenhaCriptografada = versao.SenhaCriptografada;
        senha.AtualizadoEm = DateTime.UtcNow;

        _senhaRepository.Update(senha);
        await _senhaRepository.SaveChangesAsync();

        var chave = await ObterChaveAsync(userId);
        return ToResponse(senha, chave);
    }

    private async Task<byte[]> ObterChaveAsync(int userId)
    {
        var usuario = await _usuarioRepository.GetByIdAsync(userId);
        if (usuario?.MasterPasswordSalt is not null)
        {
            // Deriva chave do salt armazenado + chave global (hybrid approach)
            // Em produção real, a master password viria do request. Aqui usamos hybrid:
            // salt serve como "personalização" da chave global por usuário.
            var salt = Convert.FromBase64String(usuario.MasterPasswordSalt);
            return DerivarChave(_chaveGlobal, salt);
        }
        return _chaveGlobal;
    }

    private static byte[] DerivarChave(byte[] chave, byte[] salt)
    {
        using var pbkdf2 = new Rfc2898DeriveBytes(chave, salt, 100_000, HashAlgorithmName.SHA256);
        return pbkdf2.GetBytes(32);
    }

    private SenhaResponse ToResponse(Senha s, byte[] chave) =>
        new(s.Id, s.Titulo, s.Login, Decrypt(s.SenhaCriptografada, chave), s.Url, s.Notas, s.Categoria, s.CriadoEm);

    private static string Encrypt(string texto, byte[] chave)
    {
        using var aes = Aes.Create();
        aes.Key = chave;
        aes.GenerateIV();

        using var encryptor = aes.CreateEncryptor();
        var bytes = Encoding.UTF8.GetBytes(texto);
        var encrypted = encryptor.TransformFinalBlock(bytes, 0, bytes.Length);

        var result = new byte[aes.IV.Length + encrypted.Length];
        aes.IV.CopyTo(result, 0);
        encrypted.CopyTo(result, aes.IV.Length);
        return Convert.ToBase64String(result);
    }

    private static string Decrypt(string cifrado, byte[] chave)
    {
        var data = Convert.FromBase64String(cifrado);
        using var aes = Aes.Create();
        aes.Key = chave;
        aes.IV = data[..16];

        using var decryptor = aes.CreateDecryptor();
        var decrypted = decryptor.TransformFinalBlock(data, 16, data.Length - 16);
        return Encoding.UTF8.GetString(decrypted);
    }
}
