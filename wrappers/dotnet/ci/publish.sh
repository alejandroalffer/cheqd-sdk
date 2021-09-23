#!/bin/bash

# -- Publish -- #
export DEV_VERSION="$DEV_BASE_VERSION-$CI_COMMIT_SHORT_SHA"

dotnet restore -p:Configuration=Release
dotnet build -c Release
dotnet pack -c Release -p:PackageVersion=$DEV_VERSION
dotnet nuget add source "$CI_SERVER_URL/api/v4/projects/$CI_PROJECT_ID/packages/nuget/index.json" --name gitlab --username gitlab-ci-token --password $CI_JOB_TOKEN --store-password-in-clear-text
dotnet nuget push "bin/Release/*.nupkg" --source gitlab
