using System.Security.Cryptography;
using System.Text;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Persistence.Data;

namespace CofreSenhas.Api;

public static class DbSeed
{
    public static async Task SeedAsync(AppDbContext context)
    {
        if (context.Usuarios.Any()) return;

        var admin = new Usuario
        {
            Nome = "Admin",
            Email = "admin@cofre.com",
            SenhaHash = BCrypt.Net.BCrypt.HashPassword("admin123"),
            CriadoEm = DateTime.UtcNow
        };
        await context.Usuarios.AddAsync(admin);
        await context.SaveChangesAsync();

        var senhas = new[]
        {
            new Senha { UsuarioId = admin.Id, Titulo = "Gmail", Login = "admin@gmail.com", SenhaCriptografada = Encrypt("MinhaS3nh@Gmail!"), Categoria = "Pessoal", CriadoEm = DateTime.UtcNow, AtualizadoEm = DateTime.UtcNow },
            new Senha { UsuarioId = admin.Id, Titulo = "GitHub", Login = "admin", SenhaCriptografada = Encrypt("GitH#b2024Seguro"), Categoria = "Trabalho", CriadoEm = DateTime.UtcNow, AtualizadoEm = DateTime.UtcNow }
        };
        await context.Senhas.AddRangeAsync(senhas);
        await context.SaveChangesAsync();
    }

    private static string Encrypt(string texto)
    {
        var key = Encoding.UTF8.GetBytes("CofreSenhas-AES-256-Key-32Chars!");
        using var aes = Aes.Create();
        aes.Key = key;
        aes.GenerateIV();

        using var encryptor = aes.CreateEncryptor();
        var bytes = Encoding.UTF8.GetBytes(texto);
        var encrypted = encryptor.TransformFinalBlock(bytes, 0, bytes.Length);

        var result = new byte[aes.IV.Length + encrypted.Length];
        aes.IV.CopyTo(result, 0);
        encrypted.CopyTo(result, aes.IV.Length);
        return Convert.ToBase64String(result);
    }
}
