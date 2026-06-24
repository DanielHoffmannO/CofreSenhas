using CofreSenhas.Domain.DTOs;
using CofreSenhas.Domain.DTOs.Senhas;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface ISenhaService
{
    Task<IEnumerable<SenhaResponse>> GetByUsuarioAsync(int userId);
    Task<PagedResponse<SenhaResponse>> GetByUsuarioPagedAsync(int userId, int page, int pageSize);
    Task<SenhaResponse?> GetByIdAsync(int id, int userId);
    Task<SenhaResponse> CriarAsync(CriarSenhaRequest request, int userId);
    Task<SenhaResponse?> AtualizarAsync(int id, CriarSenhaRequest request, int userId);
    Task<bool> DeletarAsync(int id, int userId);
    Task<IEnumerable<SenhaVersaoResponse>> GetHistoricoAsync(int senhaId, int userId);
    Task<SenhaResponse?> RestaurarVersaoAsync(int senhaId, int versaoId, int userId);
}
