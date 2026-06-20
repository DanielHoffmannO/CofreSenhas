using CofreSenhas.Domain.DTOs.Auth;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface IAuthService
{
    Task<AuthResponse> RegistrarAsync(RegisterRequest request);
    Task<AuthResponse> LoginAsync(LoginRequest request);
    Task<Setup2faResponse> Setup2faAsync(int userId);
    Task<bool> Verify2faAsync(int userId, string code);
    Task Disable2faAsync(int userId);
}
