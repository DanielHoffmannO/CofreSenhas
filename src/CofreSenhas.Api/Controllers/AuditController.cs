using System.Security.Claims;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace CofreSenhas.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class AuditController : ControllerBase
{
    private readonly IAuditService _auditService;
    public AuditController(IAuditService auditService) => _auditService = auditService;

    private int UserId => int.Parse(User.FindFirst(ClaimTypes.NameIdentifier)!.Value);

    [HttpPost]
    public async Task<IActionResult> Registrar([FromBody] AuditRequest request)
    {
        await _auditService.RegistrarAsync(UserId, request.SenhaId, request.Acao);
        return Ok();
    }

    [HttpGet]
    public async Task<IActionResult> Listar()
        => Ok(await _auditService.ListarAsync(UserId));
}

public record AuditRequest(int SenhaId, string Acao);
