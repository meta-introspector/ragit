# Submodule: `vendor/meta-introspector/ai-agent-terraform`

This submodule contains Terraform configurations for deploying AI agent infrastructure across various environments and regions, with a focus on modularity and automated checks. It manages AWS resources for AI agents, including EC2 instances, auto-scaling groups, and related IAM roles.

## Recent Commit History

### Branch: `main`
*   **Commit (f533db32fa):** `Update README.md`
    *   Removes the "Repo structure" section from the README, indicating a simplification or externalization of repository structure documentation.
*   **Commit (0919901fa7):** `Update README.md`
    *   Adds a note to the README indicating that the main branch might be out of date and to check other branches for the latest changes.
*   **Commit (26e55c1645):** `Update README.md`
    *   Renames the project from `swarms-terraform` to `ai-agent-terraform` and updates the description to be more general, supporting deployment of `elizaos/eliza`, `swarms`, or any other AI chat API infrastructure.

### Branch: `remotes/origin/feature/solana`
*   **Commit (bb536ff3a1):** `update`
    *   Updates CloudFormation templates to use "solana-node" instead of "solana-dockernode" or "agent-docker" in launch template names and auto-scaling group names, indicating a shift towards native Solana node deployments rather than Docker-based ones. Also updates the README with a note about bootstrap calling itself.
*   **Commit (0241dfd2b8):** `update`
    *   Changes the `LaunchTemplateName` in `cloudformation.yml` from `agent-docker-t4g.small-launch-template` to `solana-dockernode-t4g.small-launch-template`, further refining the naming convention for Solana-related deployments.
*   **Commit (871b60c2f8):** `update`
    *   Updates the README with setup instructions for Debian/WSL, Terraform, Docker, and bootstrapping via services repo. Modifies CloudFormation template URLs to use "zos-solfunmeme-solana-stack-template" and changes IAM role names to include "solana" instead of "tine", indicating a clear focus on Solana-specific deployments.

### Branch: `remotes/origin/feature/solfunmeme`
*   **Commit (db8077a4da):** `update readme`
    *   Adds a new ECR repository creation command for `nodemodules/sql-lite-vec` in the README, suggesting the integration of a new component for vector storage or processing.
*   **Commit (6b012ca455):** `lagging changes`
    *   Comments out a `codebuild` module in `accounts/mdupont/main.tf` and updates the `cloudposse/cicd/aws` module version, indicating a refactoring or temporary disabling of a CodeBuild pipeline and a dependency update.
*   **Commit (66e00d4e8b):** `now applied with terraform`
    *   Adds a `Readme.md` file in `accounts/mdupont/` detailing `terraform state rm` commands, suggesting a cleanup of old Terraform state related to a previous deployment.

### Branch: `remotes/origin/feature/swarms.ai`
*   **Commit (06a8b418ee):** `simple autoscale policy`
    *   Adds detailed auto-scaling policies (TargetTrackingScaling, PredictiveScaling, StepScaling) to the `autoscaling_group` module, indicating a more sophisticated approach to managing the scaling of AI agent infrastructure based on CPU utilization.
*   **Commit (13621be195):** `remove the slow asg`
    *   Comments out a "slow" auto-scaling group module (`asg_dynamic`) in `main.tf`, suggesting an optimization to use AMI-based deployments for faster scaling.
*   **Commit (45f8b38866):** `update`
    *   Updates the `Readme.md` in the `components` directory with new items like "swarms router," "fluid api," and "agent service discovery," indicating planned or ongoing development of these features. Also updates launch template naming conventions.

## Concepts Introduced/Highlighted:
*   **Terraform:** An infrastructure as code (IaC) tool used for building, changing, and versioning infrastructure safely and efficiently.
*   **CloudFormation:** AWS's infrastructure as code (IaC) service for modeling and provisioning AWS and third-party application resources.
*   **Auto Scaling Group (ASG):** An AWS service that ensures you have the correct number of Amazon EC2 instances available to handle the load for your application.
*   **Launch Template:** An Amazon EC2 feature that simplifies the process of launching instances by storing launch parameters.
*   **ECR (Elastic Container Registry):** An AWS Docker container registry that allows developers to store, manage, and deploy Docker images.
*   **SSM (Systems Manager):** An AWS service that helps you automate the operational tasks for your AWS resources.
*   **Predictive Scaling:** An AWS Auto Scaling policy that uses machine learning to predict future traffic and proactively scale resources.
*   **Step Scaling:** An AWS Auto Scaling policy that adjusts the current capacity of the Auto Scaling group based on a set of scaling adjustments, known as step adjustments.
*   **Target Tracking Scaling:** An AWS Auto Scaling policy that scales based on a target value for a specific metric.
*   **Solfunmeme:** A philosophical concept within the project that bridges formal code structures with intuitive, semantic representations.
*   **Swarms:** Refers to a collection of AI agents working collaboratively.
*   **Fluid API:** Implies a flexible and adaptable API design.
*   **Agent Service Discovery:** The mechanism by which AI agents find and communicate with each other within the deployed infrastructure.
