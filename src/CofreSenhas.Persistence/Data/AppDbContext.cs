using CofreSenhas.Domain.Entities;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Persistence.Data;

public class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options) : base(options) { }

    public DbSet<Usuario> Usuarios => Set<Usuario>();
    public DbSet<Senha> Senhas => Set<Senha>();
    public DbSet<SenhaVersao> SenhaVersoes => Set<SenhaVersao>();
    public DbSet<HistoricoGeracao> HistoricosGeracao => Set<HistoricoGeracao>();
    public DbSet<AuditLog> AuditLogs => Set<AuditLog>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.ApplyConfigurationsFromAssembly(typeof(AppDbContext).Assembly);
    }
}
