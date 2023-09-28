# Gerador de Senhas com Integração com Chat GPT

## Descrição do Projeto
Este projeto consiste em um gerador de senhas que utiliza o Chat GPT da OpenAI para criar senhas com base em palavras fornecidas pelo usuário.

## Índice
1. [Instalação](#instalação)
2. [Uso](#uso)
3. [Exemplo](#Exemplo)
4. [Créditos](#créditos)

## Instalação
Para executar este projeto, você precisa atender aos seguintes requisitos de sistema:
- C# .NET 7
- SQL Server

Siga estas etapas para configurar o projeto:
1. Clone o repositório em sua máquina local.
2. Execute os scripts SQL localizados na pasta "scripts" para configurar o banco de dados.
3. Crie uma chave de API no Chat GPT da OpenAI em [https://platform.openai.com/account/api-keys](https://platform.openai.com/account/api-keys).
4. Adicione a chave de API no endpoint POST: [https://localhost:7106/Configuracao](https://localhost:7106/Configuracao) ou utilize o Swagger para configuração em [https://localhost:7106/swagger/index.html](https://localhost:7106/swagger/index.html).
5. Use o endpoint [https://localhost:7106/Senha](https://localhost:7106/Senha) para gerar senhas com base nas palavras que você envia.

## Uso
Para gerar senhas com este projeto, siga as instruções de instalação acima e utilize o endpoint [https://localhost:7106/Senha](https://localhost:7106/Senha) após a configuração. Este projeto permite que você crie senhas personalizadas com base em palavras-chave fornecidas.

## Exemplo
**Entrada:** carro | **Saída:** C4rr0MOtOr0598

**Entrada:** Banana | **Saída:** Pr4t4B@nAn4 

## Créditos
- Documentação do Chat GPT da OpenAI: [https://platform.openai.com/docs/introduction](https://platform.openai.com/docs/introduction)
