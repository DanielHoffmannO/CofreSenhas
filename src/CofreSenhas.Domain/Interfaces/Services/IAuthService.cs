using CofreSenhas.Domain.DTOs.Auth;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface IAuthService
{
    Task<AuthResponse> RegistrarAsync(RegisterRequest request);
    Task<AuthResponse> LoginAsync(LoginRequest request);
}
