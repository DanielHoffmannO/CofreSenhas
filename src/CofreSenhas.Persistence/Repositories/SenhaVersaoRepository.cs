using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Persistence.Data;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Persistence.Repositories;

public class SenhaVersaoRepository : Repository<SenhaVersao>, ISenhaVersaoRepository
{
    public SenhaVersaoRepository(AppDbContext context) : base(context) { }

    public async Task<IEnumerable<SenhaVersao>> GetBySenhaIdAsync(int senhaId)
        => await _dbSet.Where(v => v.SenhaId == senhaId)
            .OrderByDescending(v => v.AlteradoEm)
            .ToListAsync();
}
