#!/bin/bash

# -- Release -- #
export RELEASE_VERSION=`echo $CI_COMMIT_TAG | cut -c2-`                        # pull version from tag name
dotnet restore -p:Configuration=Release
dotnet build -c Release
dotnet pack -c Release -p:PackageVersion=$RELEASE_VERSION
dotnet nuget push "bin/Release/*.nupkg" --api-key $NUGET_API_KEY --source https://api.nuget.org/v3/index.json