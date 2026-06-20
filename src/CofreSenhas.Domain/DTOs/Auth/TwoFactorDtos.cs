namespace CofreSenhas.Domain.DTOs.Auth;

public record Setup2faResponse(string Secret, string QrCodeUri);
public record Verify2faRequest(string Code);
public record LoginRequest(string Email, string Senha, string? TotpCode = null);
