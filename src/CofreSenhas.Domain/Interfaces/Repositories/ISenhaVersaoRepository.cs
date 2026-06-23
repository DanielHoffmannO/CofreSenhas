using CofreSenhas.Domain.Entities;

namespace CofreSenhas.Domain.Interfaces.Repositories;

public interface ISenhaVersaoRepository : IRepository<SenhaVersao>
{
    Task<IEnumerable<SenhaVersao>> GetBySenhaIdAsync(int senhaId);
}
