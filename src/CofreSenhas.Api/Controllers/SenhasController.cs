using System.Security.Claims;
using System.Text;
using System.Text.Json;
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

    [HttpGet("export/json")]
    public async Task<IActionResult> ExportJson()
    {
        var senhas = await _senhaService.GetByUsuarioAsync(UserId);
        var json = JsonSerializer.Serialize(senhas, new JsonSerializerOptions { WriteIndented = true });
        return File(Encoding.UTF8.GetBytes(json), "application/json", "cofre-senhas-export.json");
    }

    [HttpGet("export/csv")]
    public async Task<IActionResult> ExportCsv()
    {
        var senhas = await _senhaService.GetByUsuarioAsync(UserId);
        var sb = new StringBuilder();
        sb.AppendLine("Titulo,Login,Senha,URL,Categoria,Notas");
        foreach (var s in senhas)
            sb.AppendLine($"{CsvEscape(s.Titulo)},{CsvEscape(s.Login)},{CsvEscape(s.Senha)},{CsvEscape(s.Url)},{CsvEscape(s.Categoria)},{CsvEscape(s.Notas)}");
        return File(Encoding.UTF8.GetBytes(sb.ToString()), "text/csv", "cofre-senhas-export.csv");
    }

    private static string CsvEscape(string? value)
    {
        if (string.IsNullOrEmpty(value)) return "";
        if (value.Contains('"') || value.Contains(',') || value.Contains('\n'))
            return $"\"{value.Replace("\"", "\"\"")}\"";
        return value;
    }

    [HttpPost("import/json")]
    public async Task<IActionResult> ImportJson([FromBody] List<CriarSenhaRequest> senhas)
    {
        var count = 0;
        foreach (var s in senhas)
        {
            await _senhaService.CriarAsync(s, UserId);
            count++;
        }
        return Ok(new { imported = count });
    }
}
