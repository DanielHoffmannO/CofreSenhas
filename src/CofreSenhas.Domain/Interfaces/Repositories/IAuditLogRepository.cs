using CofreSenhas.Domain.Entities;

namespace CofreSenhas.Domain.Interfaces.Repositories;

public interface IAuditLogRepository : IRepository<AuditLog>
{
    Task<IEnumerable<AuditLog>> GetByUsuarioIdAsync(int userId, int limit = 50);
}
