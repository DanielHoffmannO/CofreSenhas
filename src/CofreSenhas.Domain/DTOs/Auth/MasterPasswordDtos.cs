namespace CofreSenhas.Domain.DTOs.Auth;

public record SetupMasterPasswordRequest(string MasterPassword);
public record VerifyMasterPasswordRequest(string MasterPassword);
public record MasterPasswordStatusResponse(bool IsConfigured);
