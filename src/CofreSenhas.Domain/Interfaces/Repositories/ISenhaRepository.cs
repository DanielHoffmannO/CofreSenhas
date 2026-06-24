using CofreSenhas.Domain.Entities;

namespace CofreSenhas.Domain.Interfaces.Repositories;

public interface ISenhaRepository : IRepository<Senha>
{
    Task<IEnumerable<Senha>> GetByUsuarioIdAsync(int userId);
    Task<(IEnumerable<Senha> Items, int TotalCount)> GetByUsuarioIdPagedAsync(int userId, int page, int pageSize);
}
