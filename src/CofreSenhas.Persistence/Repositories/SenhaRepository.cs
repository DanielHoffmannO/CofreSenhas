using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Persistence.Data;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Persistence.Repositories;

public class SenhaRepository : Repository<Senha>, ISenhaRepository
{
    public SenhaRepository(AppDbContext context) : base(context) { }

    public async Task<IEnumerable<Senha>> GetByUsuarioIdAsync(int userId)
        => await _dbSet.Where(s => s.UsuarioId == userId).ToListAsync();
}
