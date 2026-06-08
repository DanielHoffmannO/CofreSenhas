using CofreSenhas.Domain.Entities;

namespace CofreSenhas.Domain.Interfaces.Repositories;

public interface IHistoricoGeracaoRepository : IRepository<HistoricoGeracao>
{
    Task<IEnumerable<HistoricoGeracao>> GetByUsuarioIdAsync(int userId);
}
