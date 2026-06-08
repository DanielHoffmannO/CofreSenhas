namespace CofreSenhas.Domain.DTOs.Gerador;

public record GerarSenhaRequest(int Tamanho, bool UsarMaiusculas, bool UsarNumeros, bool UsarEspeciais);
