namespace CofreSenhas.Domain.Interfaces.Services;

public interface IAuditService
{
    Task RegistrarAsync(int userId, int senhaId, string acao);
    Task<IEnumerable<AuditLogResponse>> ListarAsync(int userId);
}

public record AuditLogResponse(int Id, string Acao, DateTime DataHora, string Titulo);
