using CofreSenhas.Domain.DTOs.Senhas;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface ISenhaService
{
    Task<IEnumerable<SenhaResponse>> GetByUsuarioAsync(int userId);
    Task<SenhaResponse?> GetByIdAsync(int id, int userId);
    Task<SenhaResponse> CriarAsync(CriarSenhaRequest request, int userId);
    Task<SenhaResponse?> AtualizarAsync(int id, CriarSenhaRequest request, int userId);
    Task<bool> DeletarAsync(int id, int userId);
}
