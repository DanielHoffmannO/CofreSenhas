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
            new Senha { UsuarioId = admin.Id, Titulo = "Gmail", Login = "admin@gmail.com", SenhaCriptografada = "exemplo1", CriadoEm = DateTime.UtcNow, AtualizadoEm = DateTime.UtcNow },
            new Senha { UsuarioId = admin.Id, Titulo = "GitHub", Login = "admin", SenhaCriptografada = "exemplo2", CriadoEm = DateTime.UtcNow, AtualizadoEm = DateTime.UtcNow }
        };
        await context.Senhas.AddRangeAsync(senhas);
        await context.SaveChangesAsync();
    }
}
