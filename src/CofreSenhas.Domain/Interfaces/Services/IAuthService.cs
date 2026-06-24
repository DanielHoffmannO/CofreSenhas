using CofreSenhas.Domain.DTOs.Auth;

namespace CofreSenhas.Domain.Interfaces.Services;

public interface IAuthService
{
    Task<AuthResponse> RegistrarAsync(RegisterRequest request);
    Task<AuthResponse> LoginAsync(LoginRequest request);
    Task<Setup2faResponse> Setup2faAsync(int userId);
    Task<bool> Verify2faAsync(int userId, string code);
    Task Disable2faAsync(int userId);
    Task SetupMasterPasswordAsync(int userId, string masterPassword);
    Task<bool> VerifyMasterPasswordAsync(int userId, string masterPassword);
    Task<MasterPasswordStatusResponse> GetMasterPasswordStatusAsync(int userId);
    Task<ProfileResponse> GetProfileAsync(int userId);
    Task ChangePasswordAsync(int userId, ChangePasswordRequest request);
}
