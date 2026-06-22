using CofreSenhas.Domain.Entities;
using CofreSenhas.Domain.Interfaces.Repositories;
using CofreSenhas.Domain.Interfaces.Services;

namespace CofreSenhas.Service.Services;

public class AuditService : IAuditService
{
    private readonly IAuditLogRepository _auditLogRepository;
    private readonly ISenhaRepository _senhaRepository;

    public AuditService(IAuditLogRepository auditLogRepository, ISenhaRepository senhaRepository)
    {
        _auditLogRepository = auditLogRepository;
        _senhaRepository = senhaRepository;
    }

    public async Task RegistrarAsync(int userId, int senhaId, string acao)
    {
        await _auditLogRepository.AddAsync(new AuditLog
        {
            UsuarioId = userId,
            SenhaId = senhaId,
            Acao = acao,
            DataHora = DateTime.UtcNow
        });
        await _auditLogRepository.SaveChangesAsync();
    }

    public async Task<IEnumerable<AuditLogResponse>> ListarAsync(int userId)
    {
        var logs = await _auditLogRepository.GetByUsuarioIdAsync(userId);
        var result = new List<AuditLogResponse>();

        foreach (var log in logs)
        {
            var senha = await _senhaRepository.GetByIdAsync(log.SenhaId);
            result.Add(new AuditLogResponse(log.Id, log.Acao, log.DataHora, senha?.Titulo ?? "(deletada)"));
        }

        return result;
    }
}
