# GitHub Actions Workflows

## Project Automation

The `project-automation.yml` workflow automatically manages issue status in GitHub Projects based on pull request events.

### Features

- **PR Opened**: Moves linked issues to "In Review" when a PR is opened
- **PR Approved**: Moves linked issues to "Review Approved" when a PR review is approved

### Setup Instructions

#### 1. Create a Personal Access Token (PAT)

GitHub Actions' default `GITHUB_TOKEN` cannot access GitHub Projects V2. You need to create a **Classic Personal Access Token**.

> **⚠️ Important**: Fine-grained Personal Access Tokens do **not** currently support user-owned GitHub Projects V2. They only work with organization-owned projects. Since this project is owned by a user account, you **must** use a classic token.

**Creating a Classic Personal Access Token**

1. Go to **Settings** → **Developer settings** → **Personal access tokens** → **Tokens (classic)**
   - Direct link: https://github.com/settings/tokens
2. Click **Generate new token** → **Generate new token (classic)**
3. Configure the token:
   - **Note**: `Jimbo Project Automation`
   - **Expiration**: Set to your preference (e.g., 90 days, 1 year, or custom)
   - **Select scopes**:
     - ✅ `repo` - Full control of private repositories
     - ✅ `project` - Full control of projects (**required** for Projects V2 access)
   - **Do not check**: `workflow` (not needed for this automation)
4. Click **Generate token**
5. **Copy the token immediately** (starts with `ghp_...`) - you won't be able to see it again

**For Organization-Owned Projects Only**

If your project were owned by an organization (not applicable to this repo), you could use a fine-grained token with organization-level project permissions. For user-owned projects like this one, classic tokens are the only option.

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
