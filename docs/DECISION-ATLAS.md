# Support Triage Decision Atlas

This is owner product intent, not implementation documentation. Only choices reflected in the supplied product draft and creation of this standalone repository are recorded as Decisions. Other proposed architecture details remain Questions or prototype-scoped engineering choices.

## Editing

- Decision and Question are the only leaf-entry kinds; headings group features.
- Keep stable IDs. Answer a Question with an appended Decision marked `Answers <ID>`; revise older Decisions by appending a `Supersedes <ID>` entry.
- `[x]` means independently observed, end-to-end **implemented** behavior, not agreement, scaffolding, or isolated tests.
- Keep entries concise; implementation tasks and acceptance belong in GitHub issues and executable tests.

## 1. Product and responsibility

- **1.1 [ ] Decision — Technical support investigation.**  
  Status: adopted from initial owner draft, 2026-10-14.  
  Ingest a technical-support ticket and gather evidence from relevant internal sources to produce a useful, evidence-backed assessment.

- **1.2 [ ] Decision — Privacy gateway and distinct trust zones.**  
  Status: adopted as product direction from initial owner draft, 2026-10-14.  
  Credentials, real values, authorization, vault and re-identification remain in a trusted zone. The investigator receives pseudonymized/policy-approved data only through a gateway, including follow-up tool calls. Fail closed on unsupported access.

- **1.3 [ ] Decision — Human authority over remediation.**  
  Status: adopted from initial owner draft, 2026-10-14.  
  The investigator may suggest scripts and next steps but must never automatically execute them. Treat generated scripts as inert drafts for human review.

- **1.4 [ ] Decision — Evidence-backed, structured assessments.**  
  Status: adopted from initial owner draft, 2026-10-14.  
  Include classification, suspected cause with traceable evidence, uncertainty, next steps, and gateway limitations. Without supporting evidence, classify the cause as unclear rather than asserting it.

- **1.5 [ ] Decision — Fixed support core with project profiles.**  
  Status: adopted as product direction from initial owner draft, 2026-10-14.  
  Ticket sources, evidence providers, extraction, detection and delivery vary by project through adapters/profiles; trusted gateway, investigator lifecycle, release controls and audit remain shared application behavior.

## 2. Safety and trust questions

- **2.1 [ ] Question — Which model deployment is the initial target?**  
  Status: open since 2026-10-14.  
  Local-only inference inside the customer's deployment, or a hosted model receiving only specifically approved pseudonymized evidence? The answer determines network isolation, costs and acceptable release policy.

- **2.2 [ ] Question — What permits automatic release of free-text tickets?**  
  Status: open since 2026-10-14.  
  The draft requires human review for ticket text yet also describes fully automatic triage. Define explicit categories of releasable, withheld and manually reviewed inputs before enabling real data.

- **2.3 [ ] Question — How stable should pseudonyms be?**  
  Status: open since 2026-10-14.  
  Case-scoped mappings reduce linkability; tenant-stable mappings ease cross-case analysis but need retention, deletion and collision policies.

- **2.4 [ ] Question — Which ticket source and customer environment should the first real adapter serve?**  
  Status: open since 2026-10-14.  
  Jira versus Azure DevOps, exact permitted SQL views and logs, deployment constraints and real workflow acceptance remain unselected.

- **2.5 [ ] Question — What are the externally testable privacy/security claims?**  
  Status: open since 2026-10-14.  
  Dictionary/NER scanning misses unknown identities and contextual re-identification. Define residual-risk expectations, evaluation corpus, audit retention and breach handling before claims of safe disclosure.

## 3. Future expansion

- **3.1 [ ] Question — When should external reusable engines be adopted?**  
  Status: open since 2026-10-14, not an MVP blocker.  
  Reuse NLP Stack detection, Search Engine similarity, Media Intelligence evidence composition and optional Workflow Editor authoring only when a specific vertical slice proves necessary. The application owns tenant policy and support-case lifecycle.
