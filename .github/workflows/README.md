# GitHub Actions Workflows

## Project Automation

The `project-automation.yml` workflow automatically manages issue status in GitHub Projects based on pull request events.

### Features

- **PR Opened**: Moves linked issues to "In Review" when a PR is opened
- **PR Approved**: Moves linked issues to "Review Approved" when a PR review is approved

### Setup Instructions

#### 1. Create a Personal Access Token (PAT)

GitHub Actions' default `GITHUB_TOKEN` cannot access GitHub Projects V2. You need to create a Personal Access Token (fine-grained tokens are recommended):

**Option A: Fine-grained Personal Access Token (Recommended)**

1. Go to **Settings** → **Developer settings** → **Personal access tokens** → **Fine-grained tokens**
2. Click **Generate new token**
3. Configure the token:
   - **Name**: `Jimbo Project Automation`
   - **Expiration**: Set to your preference (e.g., 90 days, 1 year, or custom)
   - **Repository access**: Select "Only select repositories" and choose `jimbo`
   - **Permissions**:
     - Repository permissions:
       - **Issues**: Read and write
       - **Pull requests**: Read and write
     - Account permissions:
       - **Projects**: Read and write
4. Click **Generate token**
5. **Copy the token immediately** (you won't be able to see it again)

**Option B: Classic Personal Access Token**

1. Go to **Settings** → **Developer settings** → **Personal access tokens** → **Tokens (classic)**
2. Click **Generate new token** → **Generate new token (classic)**
3. Give it a descriptive name: `Jimbo Project Automation`
4. Select the following scopes:
   - `project` (Full control of projects)
   - `repo` (Full control of private repositories) - if your repo is private
5. Set an appropriate expiration date
6. Click **Generate token**
7. **Copy the token immediately** (you won't be able to see it again)

#### 2. Add the Token as a Repository Secret

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `PROJECT_PAT`
5. Value: Paste the token you copied
6. Click **Add secret**

#### 3. Configure Project Number

The workflow is configured to use project number `2`. If your project has a different number:

1. Find your project number by running:
   ```bash
   gh project list --owner <your-username>
   ```
2. Edit `.github/workflows/project-automation.yml`
3. Update the `projectNumber` constant in both workflow steps

#### 4. Configure Status Field Names

The workflow expects the following status field options in your GitHub Project:

- `In Review` - Status when PR is opened
- `Review Approved` - Status when PR is approved

If your project uses different status names, update the option name strings in the workflow file.

### How It Works

1. When a PR is opened, the workflow:
   - Extracts linked issue numbers from the PR body (e.g., "Closes #1", "Fixes #2")
   - Finds those issues in the configured GitHub Project
   - Updates their status to "In Review"

2. When a PR review is approved, the workflow:
   - Extracts linked issue numbers from the PR body
   - Updates their status to "Review Approved"

### Troubleshooting

**Error: "Could not resolve to a ProjectV2 with the number X"**
- Verify the project number is correct
- Ensure the `PROJECT_PAT` secret is set correctly
- Check that the PAT has the `project` scope

**Error: "In Review status not found in project"**
- Verify your project has a "Status" field
- Check that the status field has an option named "In Review" (exact match, case-sensitive)

**Workflow runs but doesn't update project**
- Check that issues are properly linked in the PR body using keywords: `Closes #X`, `Fixes #X`, or `Resolves #X`
- Verify the issues are actually in the configured project
