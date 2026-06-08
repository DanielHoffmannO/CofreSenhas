using CofreSenhas.Domain.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace CofreSenhas.Persistence.Configurations;

public class SenhaConfiguration : IEntityTypeConfiguration<Senha>
{
    public void Configure(EntityTypeBuilder<Senha> builder)
    {
        builder.HasKey(s => s.Id);
        builder.Property(s => s.Titulo).IsRequired().HasMaxLength(200);
        builder.Property(s => s.Login).IsRequired().HasMaxLength(200);
        builder.Property(s => s.SenhaCriptografada).IsRequired();
        builder.Property(s => s.Url).HasMaxLength(500);

        builder.HasOne(s => s.Usuario)
            .WithMany()
            .HasForeignKey(s => s.UsuarioId)
            .OnDelete(DeleteBehavior.Restrict);
    }
}
