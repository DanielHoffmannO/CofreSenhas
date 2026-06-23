using CofreSenhas.Domain.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace CofreSenhas.Persistence.Configurations;

public class SenhaVersaoConfiguration : IEntityTypeConfiguration<SenhaVersao>
{
    public void Configure(EntityTypeBuilder<SenhaVersao> builder)
    {
        builder.HasKey(v => v.Id);
        builder.Property(v => v.SenhaCriptografada).IsRequired();
        builder.Property(v => v.Titulo).IsRequired().HasMaxLength(200);
        builder.Property(v => v.Login).IsRequired().HasMaxLength(200);

        builder.HasOne(v => v.Senha)
            .WithMany()
            .HasForeignKey(v => v.SenhaId)
            .OnDelete(DeleteBehavior.Cascade);
    }
}
