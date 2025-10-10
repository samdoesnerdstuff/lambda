# Security Policy for Lambda

## Supported Versions
Lambda is currently in active development. Only the latest commit on the `main` branch is considered supported at this time.

Once formal releases begin, this section will list supported versions for security fixes.

| Version | Supported |
|----------|------------|
| main (development) | ✅ |
| pre-release tags (v0.x) | 🛠️ Varies<sup>1</sup> |
| release tags (v1.x) | 🛠️ Varies<sup>2</sup> |
| older commits | ❌ |

<sup>1</sup>Pre-release versions (v0.x) may not receive security updates or backports. \
<sup>2</sup>Only the latest stable release in each major series (v1.x) is actively maintained.
For changes such as (v1.x)->(v2.x), all prior versions are immediately marked EOL and will not recieve prioritized updates.

---

## Reporting a Vulnerability
If you believe you’ve found a security or memory-safety issue in the Lambda compiler or standard library:

1. **Do not** create a public GitHub issue.
2. Create a [GitHub Security Advisory](https://github.com/samdoesnerdstuff/lambda/security)
3. Include as much detail as possible — affected files, minimal reproduction steps, and potential impact.

You’ll receive a confirmation within **10 days**, and we’ll coordinate a fix or disclosure timeline if needed. \
This timeline is long, since as of October 2025, I (samdoesnerdstuff) am the only person who can access these advisories.

---

## Disclosure Process
- Verified issues are fixed privately.
- A public advisory and patch are released together.
- Credits are given to the reporter unless anonymity is requested.

Thank you for helping keep Lambda secure and (mostly) stable!
