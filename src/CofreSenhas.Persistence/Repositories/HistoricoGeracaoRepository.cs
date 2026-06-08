using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Persistence.Data;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Persistence.Repositories;

public class HistoricoGeracaoRepository : Repository<HistoricoGeracao>, IHistoricoGeracaoRepository
{
    public HistoricoGeracaoRepository(AppDbContext context) : base(context) { }

    public async Task<IEnumerable<HistoricoGeracao>> GetByUsuarioIdAsync(int userId)
        => await _dbSet.Where(h => h.UsuarioId == userId).ToListAsync();
}
