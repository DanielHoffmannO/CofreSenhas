FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
WORKDIR /src
COPY CofreSenhas.sln .
COPY src/CofreSenhas.Domain/*.csproj src/CofreSenhas.Domain/
COPY src/CofreSenhas.Service/*.csproj src/CofreSenhas.Service/
COPY src/CofreSenhas.Persistence/*.csproj src/CofreSenhas.Persistence/
COPY src/CofreSenhas.Api/*.csproj src/CofreSenhas.Api/
COPY tests/CofreSenhas.Tests/*.csproj tests/CofreSenhas.Tests/
RUN dotnet restore
COPY . .
RUN dotnet publish src/CofreSenhas.Api -c Release -o /app

FROM mcr.microsoft.com/dotnet/aspnet:9.0
WORKDIR /app
COPY --from=build /app .
EXPOSE 5000
ENV ASPNETCORE_URLS=http://+:5000
ENTRYPOINT ["dotnet", "CofreSenhas.Api.dll"]
