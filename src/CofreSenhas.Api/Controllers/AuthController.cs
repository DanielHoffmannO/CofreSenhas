using System.Security.Claims;
using CofreSenhas.Domain.DTOs.Auth;
using CofreSenhas.Domain.Interfaces.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.RateLimiting;

namespace CofreSenhas.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
public class AuthController : ControllerBase
{
    private readonly IAuthService _authService;
    public AuthController(IAuthService authService) => _authService = authService;

    private int UserId => int.Parse(User.FindFirst(ClaimTypes.NameIdentifier)!.Value);

    [HttpPost("register")]
    [AllowAnonymous]
    public async Task<IActionResult> Register(RegisterRequest request)
    {
        var result = await _authService.RegistrarAsync(request);
        return Ok(result);
    }

    [HttpPost("login")]
    [AllowAnonymous]
    [EnableRateLimiting("login")]
    public async Task<IActionResult> Login(LoginRequest request)
    {
        try
        {
            var result = await _authService.LoginAsync(request);
            return Ok(result);
        }
        catch (InvalidOperationException ex) when (ex.Message == "2FA_REQUIRED")
        {
            return Ok(new { twoFactorRequired = true });
        }
        catch (UnauthorizedAccessException ex)
        {
            return Unauthorized(new { message = ex.Message });
        }
    }

    [HttpPost("2fa/setup")]
    [Authorize]
    public async Task<IActionResult> Setup2fa()
    {
        var result = await _authService.Setup2faAsync(UserId);
        return Ok(result);
    }

    [HttpPost("2fa/verify")]
    [Authorize]
    public async Task<IActionResult> Verify2fa(Verify2faRequest request)
    {
        var success = await _authService.Verify2faAsync(UserId, request.Code);
        return success ? Ok(new { message = "2FA ativado com sucesso!" }) : BadRequest(new { message = "Código inválido." });
    }

    [HttpPost("2fa/disable")]
    [Authorize]
    public async Task<IActionResult> Disable2fa()
    {
        await _authService.Disable2faAsync(UserId);
        return Ok(new { message = "2FA desativado." });
    }

    [HttpPost("master-password/setup")]
    [Authorize]
    public async Task<IActionResult> SetupMasterPassword(SetupMasterPasswordRequest request)
    {
        await _authService.SetupMasterPasswordAsync(UserId, request.MasterPassword);
        return Ok(new { message = "Master password configurada com sucesso!" });
    }

    [HttpPost("master-password/verify")]
    [Authorize]
    public async Task<IActionResult> VerifyMasterPassword(VerifyMasterPasswordRequest request)
    {
        var valid = await _authService.VerifyMasterPasswordAsync(UserId, request.MasterPassword);
        return valid ? Ok(new { valid = true }) : BadRequest(new { valid = false, message = "Master password incorreta." });
    }

    [HttpGet("master-password/status")]
    [Authorize]
    public async Task<IActionResult> GetMasterPasswordStatus()
        => Ok(await _authService.GetMasterPasswordStatusAsync(UserId));
}
