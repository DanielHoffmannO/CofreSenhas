namespace CofreSenhas.Domain.DTOs.Auth;

public record ProfileResponse(int Id, string Nome, string Email, DateTime CriadoEm, bool TwoFactorEnabled, bool MasterPasswordConfigured);
public record ChangePasswordRequest(string SenhaAtual, string NovaSenha);
