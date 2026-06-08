using System.Security.Claims;
using CofreSenhas.Domain.DTOs.Gerador;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace CofreSenhas.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class GeradorController : ControllerBase
{
    private readonly IGeradorSenhaService _geradorService;
    public GeradorController(IGeradorSenhaService geradorService) => _geradorService = geradorService;

    private int UserId => int.Parse(User.FindFirst(ClaimTypes.NameIdentifier)!.Value);

    [HttpPost]
    public async Task<IActionResult> Gerar(GerarSenhaRequest request)
        => Ok(await _geradorService.GerarAsync(request, UserId));
}
