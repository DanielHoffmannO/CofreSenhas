using System.Security.Claims;
using CofreSenhas.Domain.Entities;
using CofreSenhas.Persistence.Data;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace CofreSenhas.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class AuditController : ControllerBase
{
    private readonly AppDbContext _db;
    public AuditController(AppDbContext db) => _db = db;

    private int UserId => int.Parse(User.FindFirst(ClaimTypes.NameIdentifier)!.Value);

    /// <summary>Registra uma ação de auditoria (ex: senha visualizada ou copiada)</summary>
    [HttpPost]
    public async Task<IActionResult> Registrar([FromBody] AuditRequest request)
    {
        _db.AuditLogs.Add(new AuditLog
        {
            UsuarioId = UserId,
            SenhaId = request.SenhaId,
            Acao = request.Acao,
            DataHora = DateTime.UtcNow
        });
        await _db.SaveChangesAsync();
        return Ok();
    }

    /// <summary>Lista os últimos 50 registros de auditoria do usuário</summary>
    [HttpGet]
    public async Task<IActionResult> Listar()
    {
        var logs = await _db.AuditLogs
            .Where(a => a.UsuarioId == UserId)
            .OrderByDescending(a => a.DataHora)
            .Take(50)
            .Join(_db.Senhas, a => a.SenhaId, s => s.Id, (a, s) => new { a.Id, a.Acao, a.DataHora, Titulo = s.Titulo })
            .ToListAsync();
        return Ok(logs);
    }
}

public record AuditRequest(int SenhaId, string Acao);
