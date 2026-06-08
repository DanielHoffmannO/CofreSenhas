using System.Security.Claims;
using CofreSenhas.Domain.DTOs.Senhas;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace CofreSenhas.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class SenhasController : ControllerBase
{
    private readonly ISenhaService _senhaService;
    public SenhasController(ISenhaService senhaService) => _senhaService = senhaService;

    private int UserId => int.Parse(User.FindFirst(ClaimTypes.NameIdentifier)!.Value);

    [HttpGet]
    public async Task<IActionResult> GetAll()
        => Ok(await _senhaService.GetByUsuarioAsync(UserId));

    [HttpGet("{id}")]
    public async Task<IActionResult> GetById(int id)
    {
        var result = await _senhaService.GetByIdAsync(id, UserId);
        return result is null ? NotFound() : Ok(result);
    }

    [HttpPost]
    public async Task<IActionResult> Create(CriarSenhaRequest request)
        => Created("", await _senhaService.CriarAsync(request, UserId));

    [HttpPut("{id}")]
    public async Task<IActionResult> Update(int id, CriarSenhaRequest request)
    {
        var result = await _senhaService.AtualizarAsync(id, request, UserId);
        return result is null ? NotFound() : Ok(result);
    }

    [HttpDelete("{id}")]
    public async Task<IActionResult> Delete(int id)
        => await _senhaService.DeletarAsync(id, UserId) ? NoContent() : NotFound();
}
