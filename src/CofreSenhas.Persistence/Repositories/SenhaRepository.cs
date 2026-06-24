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

    public async Task<(IEnumerable<Senha> Items, int TotalCount)> GetByUsuarioIdPagedAsync(int userId, int page, int pageSize)
    {
        var query = _dbSet.Where(s => s.UsuarioId == userId).OrderByDescending(s => s.AtualizadoEm);
        var total = await query.CountAsync();
        var items = await query.Skip((page - 1) * pageSize).Take(pageSize).ToListAsync();
        return (items, total);
    }
}
