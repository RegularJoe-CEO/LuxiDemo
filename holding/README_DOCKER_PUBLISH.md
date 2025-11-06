# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

# Docker Image Publishing Instructions

This document explains how to publish the Docker image to GitHub Container Registry (GHCR).

## Prerequisites

- GitHub repository with `packages: write` permission
- Workflows are configured (`.github/workflows/publish-on-release.yml` and `release.yml`)
- Repository is public or you have appropriate permissions

## Method 1: Automatic Publishing via Git Tag (Recommended)

The easiest way to publish is to create a version tag:

```bash
# Ensure you're on the main branch and up to date
git checkout main
git pull

# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0
```

This will automatically trigger **both** workflows:
- `publish-on-release.yml` - Triggered by tags matching `v*`
- `release.yml` - Triggered by tags matching `v*.*.*`

The image will be published to:
- `ghcr.io/regularjoe-ceo/luxi-edge:latest`
- `ghcr.io/regularjoe-ceo/luxi-edge:v0.1.0`
- `ghcr.io/regularjoe-ceo/luxi-edge:sha-<commit-hash>`

## Method 2: Manual Workflow Trigger

If you don't want to create a tag yet:

1. Go to: https://github.com/RegularJoe-CEO/LuxiEdge/actions
2. Click on "Publish Docker image" workflow
3. Click "Run workflow" dropdown
4. Select the branch (usually `main`)
5. Click "Run workflow"

## Method 3: Local Build and Push (Manual)

```bash
# Log in to GHCR
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Build for multiple platforms
docker buildx create --use
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ghcr.io/regularjoe-ceo/luxi-edge:latest \
  -t ghcr.io/regularjoe-ceo/luxi-edge:v0.1.0 \
  --push .
```

## Verifying the Published Image

After publishing, verify the image is available:

```bash
# Pull the image
docker pull ghcr.io/regularjoe-ceo/luxi-edge:latest

# Run it
docker run -p 8080:8080 ghcr.io/regularjoe-ceo/luxi-edge:latest

# Test
curl http://localhost:8080/health
```

## Viewing Published Packages

1. Go to: https://github.com/orgs/RegularJoe-CEO/packages
2. Or: https://github.com/RegularJoe-CEO/LuxiEdge/pkgs/container/luxi-edge
3. Click on the package to see all versions

## Making the Package Public

After first publish, the package may be private by default:

1. Go to the package page
2. Click "Package settings"
3. Scroll to "Danger Zone"
4. Click "Change visibility"
5. Select "Public"
6. Confirm

## Current Status

**⚠️ BEFORE FIRST PUBLISH:**
- Documentation mentions images are "illustrative only"
- Need to publish first image

**✅ AFTER FIRST PUBLISH:**
- Update `docs/ARCHITECTURE.md` - Remove "illustrative only" notes
- Verify all docs use `ghcr.io/regularjoe-ceo/luxi-edge:latest`
- Test the published image

## Recommended First Publish

```bash
# Create the first release tag
git tag -a v0.1.0 -m "Initial release - Docker image for Luxi Edge"
git push origin v0.1.0

# This will trigger the workflows and publish:
# - ghcr.io/regularjoe-ceo/luxi-edge:latest
# - ghcr.io/regularjoe-ceo/luxi-edge:v0.1.0
```

## Troubleshooting

**"Permission denied" errors:**
- Ensure the repository has `packages: write` permission
- Check that GITHUB_TOKEN secret is available in Actions

**"Platform not supported" errors:**
- The workflows build for both amd64 and arm64
- If one fails, check the workflow logs

**Image not showing up:**
- Wait a few minutes after workflow completes
- Check the Actions tab for any errors
- Verify package visibility is set to Public
