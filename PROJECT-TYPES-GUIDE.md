# Project Types Configuration Guide

This guide explains how to configure the README.md template for different project types.

## Available Project Types

### 1. CLI Tools (Command Line Interface)

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "Using as a CLI Tool"
{{project_type_section_slug}} = "using-as-a-cli-tool"
{{PROJECT_TYPE_INTRO}} = "For command-line users, this project provides a powerful CLI interface."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = "For end-users, {{PROJECT_NAME}} ships with **`{{CLI_PACKAGE_NAME}}`**, {{CLI_DESCRIPTION}}."
{{PROJECT_TYPE_SECTION_INTRO}} = "The CLI provides a straightforward interface to interact with {{PROJECT_NAME}}."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "The CLI can be installed globally or used locally in your project."
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```bash\nnpm install -g {{CLI_PACKAGE_NAME}}\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{CLI_PACKAGE_NAME}}`**: {{CLI_COMPONENT_DESCRIPTION}}\n- **`{{SDK_PACKAGE_NAME}}`**: {{SDK_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "Core Lifecycle"
{{lifecycle_or_workflow_section_slug}} = "core-lifecycle"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "{{LIFECYCLE_INTRO}}"
{{USAGE_OR_API_SECTION}} = "Command Reference"
{{usage_or_api_section_slug}} = "command-reference"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Installation Guide](docs/wiki/Installation.md)**\n*   **[CLI Command Reference](docs/wiki/CLI-Guide.md)**\n*   **[Technical Architecture](docs/wiki/Architecture.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "| Full Command | Short Alias | Description |\n| :--- | :--- | :--- |\n| `{{CLI_NAME}} {{CMD_1}}` | `{{ALIAS_1}}` | {{CMD_1_DESC}} |\n| `{{CLI_NAME}} {{CMD_2}}` | `{{ALIAS_2}}` | {{CMD_2_DESC}} |\n| `{{CLI_NAME}} config` | `c` | Manage configuration. |\n| `{{CLI_NAME}} logs` | `l` | View application logs. |\n| `{{CLI_NAME}} about` | `a` | Display version info. |"
```

### 2. Web Applications

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "Getting Started"
{{project_type_section_slug}} = "getting-started"
{{PROJECT_TYPE_INTRO}} = "This is a modern web application built with {{FRAMEWORK}}."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = "The application includes a responsive frontend and robust backend services."
{{PROJECT_TYPE_SECTION_INTRO}} = "Follow these steps to get the application running locally."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "The application can be run locally or deployed to your preferred hosting platform."
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```bash\ngit clone https://github.com/{{GITHUB_USER}}/{{REPO_NAME}}.git\ncd {{REPO_NAME}}\nnpm install\nnpm run dev\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{FRONTEND_PACKAGE_NAME}}`**: {{FRONTEND_COMPONENT_DESCRIPTION}}\n- **`{{BACKEND_PACKAGE_NAME}}`**: {{BACKEND_COMPONENT_DESCRIPTION}}\n- **`{{API_PACKAGE_NAME}}`**: {{API_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "Application Workflow"
{{lifecycle_or_workflow_section_slug}} = "application-workflow"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "The application follows this workflow from user interaction to data processing."
{{USAGE_OR_API_SECTION}} = "API Reference"
{{usage_or_api_section_slug}} = "api-reference"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Installation Guide](docs/wiki/Installation.md)**\n*   **[API Documentation](docs/wiki/API.md)**\n*   **[Frontend Guide](docs/wiki/Frontend.md)**\n*   **[Deployment Guide](docs/wiki/Deployment.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "### REST API Endpoints\n\n| Method | Endpoint | Description |\n| :--- | :--- | :--- |\n| GET | `/api/{{RESOURCE_1}}` | List all {{RESOURCE_1}} |\n| POST | `/api/{{RESOURCE_1}}` | Create new {{RESOURCE_1}} |\n| GET | `/api/{{RESOURCE_1}}/:id` | Get {{RESOURCE_1}} by ID |\n| PUT | `/api/{{RESOURCE_1}}/:id` | Update {{RESOURCE_1}} |\n| DELETE | `/api/{{RESOURCE_1}}/:id` | Delete {{RESOURCE_1}} |"
```

### 3. Libraries/SDKs

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "Using as a Library"
{{project_type_section_slug}} = "using-as-a-library"
{{PROJECT_TYPE_INTRO}} = "{{PROJECT_NAME}} is designed to be embedded in other projects."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = ""
{{PROJECT_TYPE_SECTION_INTRO}} = "Unlike {{COMPETING_APPROACH}}, **{{PROJECT_NAME}} is designed to be embedded**. If you are building {{USE_CASE_EXAMPLES}}, you can drop `{{CORE_PACKAGE_NAME}}` directly into your project."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "Add it to your `{{DEPENDENCY_FILE}}`:"
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```{{LANGUAGE_SLUG}}\n{{DEPENDENCY_SNIPPET}}\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{UTILS_PACKAGE_NAME}}`**: {{UTILS_COMPONENT_DESCRIPTION}}\n- **`{{TYPES_PACKAGE_NAME}}`**: {{TYPES_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "Core Lifecycle"
{{lifecycle_or_workflow_section_slug}} = "core-lifecycle"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "The library follows this lifecycle when processing data."
{{USAGE_OR_API_SECTION}} = "API Reference"
{{usage_or_api_section_slug}} = "api-reference"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Installation Guide](docs/wiki/Installation.md)**\n*   **[API Reference](docs/wiki/API.md)**\n*   **[Examples](docs/wiki/Examples.md)**\n*   **[Migration Guide](docs/wiki/Migration.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "### Core API\n\n```javascript\nimport { {{CORE_CLASS_NAME}} } from '{{CORE_PACKAGE_NAME}}';\n\nconst instance = new {{CORE_CLASS_NAME}}({{CONFIG_OPTIONS}});\nawait instance.{{METHOD_NAME}}();\n```\n\n### Available Methods\n\n| Method | Parameters | Returns | Description |\n| :--- | :--- | :--- | :--- |\n| `{{METHOD_1}}` | `{{PARAMS_1}}` | `{{RETURN_1}}` | {{METHOD_1_DESC}} |\n| `{{METHOD_2}}` | `{{PARAMS_2}}` | `{{RETURN_2}}` | {{METHOD_2_DESC}} |\n| `{{METHOD_3}}` | `{{PARAMS_3}}` | `{{RETURN_3}}` | {{METHOD_3_DESC}} |"
```

### 4. APIs (REST/GraphQL)

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "API Overview"
{{project_type_section_slug}} = "api-overview"
{{PROJECT_TYPE_INTRO}} = "{{PROJECT_NAME}} provides a powerful {{API_TYPE}} API for developers."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = "The API is designed to be simple, intuitive, and well-documented."
{{PROJECT_TYPE_SECTION_INTRO}} = "The API follows RESTful principles and provides endpoints for all major operations."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "You can interact with the API using any HTTP client or our official SDK."
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```bash\n# Using cURL\ncurl -X GET https://api.{{DOMAIN}}/{{ENDPOINT}}\n\n# Using the SDK\nnpm install {{SDK_PACKAGE_NAME}}\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{API_PACKAGE_NAME}}`**: {{API_COMPONENT_DESCRIPTION}}\n- **`{{MIDDLEWARE_PACKAGE_NAME}}`**: {{MIDDLEWARE_COMPONENT_DESCRIPTION}}\n- **`{{AUTH_PACKAGE_NAME}}`**: {{AUTH_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "Request Lifecycle"
{{lifecycle_or_workflow_section_slug}} = "request-lifecycle"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "All API requests follow this lifecycle from authentication to response."
{{USAGE_OR_API_SECTION}} = "API Endpoints"
{{usage_or_api_section_slug}} = "api-endpoints"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Authentication](docs/wiki/Authentication.md)**\n*   **[API Reference](docs/wiki/API.md)**\n*   **[Rate Limiting](docs/wiki/Rate-Limiting.md)**\n*   **[Webhooks](docs/wiki/Webhooks.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "### Base URL\n```\nhttps://api.{{DOMAIN}}/v{{VERSION}}\n```\n\n### Authentication\n\nAll requests must include an API key in the header:\n```\nAuthorization: Bearer {{API_KEY_PLACEHOLDER}}\n```\n\n### Endpoints\n\n| Method | Endpoint | Description |\n| :--- | :--- | :--- |\n| GET | `/{{RESOURCE_1}}` | List all {{RESOURCE_1}} |\n| POST | `/{{RESOURCE_1}}` | Create {{RESOURCE_1}} |\n| GET | `/{{RESOURCE_1}}/:id` | Get {{RESOURCE_1}} by ID |\n| PUT | `/{{RESOURCE_1}}/:id` | Update {{RESOURCE_1}} |\n| DELETE | `/{{RESOURCE_1}}/:id` | Delete {{RESOURCE_1}} |"
```

### 5. Mobile Applications

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "Getting Started"
{{project_type_section_slug}} = "getting-started"
{{PROJECT_TYPE_INTRO}} = "{{PROJECT_NAME}} is a cross-platform mobile application built with {{FRAMEWORK}}."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = "The application supports both iOS and Android platforms."
{{PROJECT_TYPE_SECTION_INTRO}} = "Follow these steps to set up the development environment and run the app."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "The app can be run on emulators, simulators, or physical devices."
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```bash\n# Clone the repository\ngit clone https://github.com/{{GITHUB_USER}}/{{REPO_NAME}}.git\ncd {{REPO_NAME}}\n\n# Install dependencies\nnpm install\n\n# Run on iOS\nnpm run ios\n\n# Run on Android\nnpm run android\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{APP_PACKAGE_NAME}}`**: {{APP_COMPONENT_DESCRIPTION}}\n- **`{{SHARED_PACKAGE_NAME}}`**: {{SHARED_COMPONENT_DESCRIPTION}}\n- **`{{NATIVE_PACKAGE_NAME}}`**: {{NATIVE_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "App Flow"
{{lifecycle_or_workflow_section_slug}} = "app-flow"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "The application follows this flow from launch to user interaction."
{{USAGE_OR_API_SECTION}} = "User Guide"
{{usage_or_api_section_slug}} = "user-guide"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Setup Guide](docs/wiki/Setup.md)**\n*   **[Development Guide](docs/wiki/Development.md)**\n*   **[Deployment Guide](docs/wiki/Deployment.md)**\n*   **[Testing Guide](docs/wiki/Testing.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "### Key Screens\n\n| Screen | Description | Features |\n| :--- | :--- | :--- |\n| {{SCREEN_1}} | {{SCREEN_1_DESC}} | {{SCREEN_1_FEATURES}} |\n| {{SCREEN_2}} | {{SCREEN_2_DESC}} | {{SCREEN_2_FEATURES}} |\n| {{SCREEN_3}} | {{SCREEN_3_DESC}} | {{SCREEN_3_FEATURES}} |\n\n### Navigation\n\nThe app uses {{NAVIGATION_TYPE}} for navigation between screens."
```

### 6. Desktop Applications

**Configuration:**

```markdown
{{PROJECT_TYPE_SECTION}} = "Getting Started"
{{project_type_section_slug}} = "getting-started"
{{PROJECT_TYPE_INTRO}} = "{{PROJECT_NAME}} is a desktop application built with {{FRAMEWORK}}."
{{PROJECT_TYPE_SPECIFIC_INTRO}} = "The application supports Windows, macOS, and Linux platforms."
{{PROJECT_TYPE_SECTION_INTRO}} = "Follow these steps to build and run the application on your platform."
{{EMBEDDABLE_OR_STANDALONE_INTRO}} = "The application can be built for distribution or run in development mode."
{{DEPENDENCY_INSTALLATION_BLOCK}} = "```bash\n# Clone the repository\ngit clone https://github.com/{{GITHUB_USER}}/{{REPO_NAME}}.git\ncd {{REPO_NAME}}\n\n# Install dependencies\nnpm install\n\n# Run in development\nnpm run dev\n\n# Build for production\nnpm run build\n```"
{{ADDITIONAL_COMPONENTS_BLOCK}} = "- **`{{MAIN_PACKAGE_NAME}}`**: {{MAIN_COMPONENT_DESCRIPTION}}\n- **`{{RENDERER_PACKAGE_NAME}}`**: {{RENDERER_COMPONENT_DESCRIPTION}}\n- **`{{NATIVE_PACKAGE_NAME}}`**: {{NATIVE_COMPONENT_DESCRIPTION}}"
{{LIFECYCLE_OR_WORKFLOW_SECTION}} = "Application Flow"
{{lifecycle_or_workflow_section_slug}} = "application-flow"
{{LIFECYCLE_OR_WORKFLOW_INTRO}} = "The application follows this flow from launch to user interaction."
{{USAGE_OR_API_SECTION}} = "User Guide"
{{usage_or_api_section_slug}} = "user-guide"
{{DOCUMENTATION_LINKS_BLOCK}} = "*   **[Installation Guide](docs/wiki/Installation.md)**\n*   **[Building Guide](docs/wiki/Building.md)**\n*   **[Packaging Guide](docs/wiki/Packaging.md)**\n*   **[Platform-Specific Notes](docs/wiki/Platforms.md)**"
{{USAGE_OR_API_CONTENT_BLOCK}} = "### Supported Platforms\n\n| Platform | Status | Notes |\n| :--- | :--- | :--- |\n| Windows | ✅ | Tested on Windows 10/11 |\n| macOS | ✅ | Tested on macOS 11+ |\n| Linux | ✅ | Tested on Ubuntu 20.04+ |\n\n### System Requirements\n\n- **OS**: {{OS_REQUIREMENTS}}\n- **RAM**: {{RAM_REQUIREMENTS}}\n- **Disk Space**: {{DISK_REQUIREMENTS}}\n- **Dependencies**: {{DEPENDENCY_REQUIREMENTS}}"
```

## Quick Setup Script

You can use this script to automatically replace placeholders based on your project type:

```bash
#!/bin/bash
PROJECT_TYPE=$1

case $PROJECT_TYPE in
  cli)
    # Replace CLI-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/Using as a CLI Tool/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/Core Lifecycle/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/Command Reference/g' README.md
    ;;
  web)
    # Replace Web-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/Getting Started/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/Application Workflow/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/API Reference/g' README.md
    ;;
  library)
    # Replace Library-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/Using as a Library/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/Core Lifecycle/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/API Reference/g' README.md
    ;;
  api)
    # Replace API-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/API Overview/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/Request Lifecycle/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/API Endpoints/g' README.md
    ;;
  mobile)
    # Replace Mobile-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/Getting Started/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/App Flow/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/User Guide/g' README.md
    ;;
  desktop)
    # Replace Desktop-specific placeholders
    sed -i 's/{{PROJECT_TYPE_SECTION}}/Getting Started/g' README.md
    sed -i 's/{{LIFECYCLE_OR_WORKFLOW_SECTION}}/Application Flow/g' README.md
    sed -i 's/{{USAGE_OR_API_SECTION}}/User Guide/g' README.md
    ;;
  *)
    echo "Unknown project type: $PROJECT_TYPE"
    echo "Supported types: cli, web, library, api, mobile, desktop"
    exit 1
    ;;
esac

echo "README.md configured for $PROJECT_TYPE project"
```

## Customization Tips

1. **Mix and Match**: You can combine elements from different project types. For example, a web app with a CLI admin tool.

2. **Conditional Sections**: Use comment blocks to include/exclude sections based on project type:
   ```markdown
   <!-- CLI_ONLY_START -->
   ## Command Reference
   <!-- CLI_ONLY_END -->
   ```

3. **Architecture Diagrams**: Adjust the Mermaid diagrams to match your actual architecture.

4. **Documentation Links**: Update the documentation links to match your actual documentation structure.

5. **Badges**: Add project-type-specific badges (e.g., mobile app stores, web deployment platforms).
