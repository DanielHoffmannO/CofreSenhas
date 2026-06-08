using CofreSenhas.Domain.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace CofreSenhas.Persistence.Configurations;

public class HistoricoGeracaoConfiguration : IEntityTypeConfiguration<HistoricoGeracao>
{
    public void Configure(EntityTypeBuilder<HistoricoGeracao> builder)
    {
        builder.HasKey(h => h.Id);
        builder.Property(h => h.Prompt).IsRequired().HasMaxLength(500);
        builder.Property(h => h.SenhaGerada).IsRequired().HasMaxLength(200);

        builder.HasOne(h => h.Usuario)
            .WithMany()
            .HasForeignKey(h => h.UsuarioId)
            .OnDelete(DeleteBehavior.Restrict);
    }
}
