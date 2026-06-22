using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Persistence.Data;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Persistence.Repositories;

public class AuditLogRepository : Repository<AuditLog>, IAuditLogRepository
{
    public AuditLogRepository(AppDbContext context) : base(context) { }

    public async Task<IEnumerable<AuditLog>> GetByUsuarioIdAsync(int userId, int limit = 50)
        => await _dbSet
            .Where(a => a.UsuarioId == userId)
            .OrderByDescending(a => a.DataHora)
            .Take(limit)
            .ToListAsync();
}
