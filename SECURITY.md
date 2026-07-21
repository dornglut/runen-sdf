# Security Policy

## Reporting a vulnerability

Do not disclose a suspected vulnerability in a public issue.

Use GitHub's private vulnerability reporting for `Crystonix/runen-sdf` when it is available. Otherwise contact the repository owner privately through the contact method listed on the owner's GitHub profile and identify the affected revision, impact, reproduction conditions, and any proposed mitigation.

Reports are acknowledged after they can be reviewed. No response-time or disclosure deadline is guaranteed while the package remains unpublished.

## Supported versions

RunenSDF is currently unpublished and pre-release. Security fixes are applied to the default branch and to any exact revision explicitly consumed by Runenwerk. No broader support window is promised until a release policy is accepted.

## Scope

Relevant reports include memory-safety defects, validation bypasses, denial-of-service behavior from untrusted field implementations or inputs, dependency compromise, and incorrect numerical behavior that violates documented conservative-step guarantees.
