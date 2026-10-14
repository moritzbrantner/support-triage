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

- **2.6 [ ] Decision — Hosted-model-first initial investigator.**  
  Status: adopted by owner 2026-10-14; Answers 2.1.  
  The first AI-backed investigator targets an external hosted model, not mandatory local inference. Only evidence explicitly approved for release by the trusted gateway may be sent to that provider, including follow-up tool results; credentials, raw customer values, and the vault remain in the trusted zone. This chooses the deployment direction, not a provider, production-data authorization, or a requirement for the synthetic-only demonstration to make real external calls. Local inference may be supported later.

- **2.2 [ ] Question — What permits automatic release of free-text tickets?**  
  Status: open since 2026-10-14.  
  The draft requires human review for ticket text yet also describes fully automatic triage. Define explicit categories of releasable, withheld and manually reviewed inputs before enabling real data.

- **2.7 [ ] Decision — Automatic policy-gated release of ticket free text.**  
  Status: adopted by owner 2026-10-14; Answers 2.2.  
  Ticket descriptions and comments may reach the hosted investigator automatically only when a trusted, validated release policy explicitly approves the transformed content. Uncertain, unsupported, or rejected material must be withheld from the model and flagged for human review rather than silently released. This removes routine per-ticket review for approved material; it does not establish anonymization, select the policy's detection thresholds, or authorize processing real customer data before the gateway and deployment safeguards are validated.

- **2.11 [ ] Question — Continue or pause an investigation when some evidence is withheld?**  
  Status: open since 2026-10-14; elaborates the unapproved portion of Decision 2.7.  
  For a ticket with both gateway-approved evidence and text withheld pending human review, should the investigator continue with the approved evidence and explicitly report missing information, or pause the investigation until a human reviews the withheld material? In either case, withheld content never reaches the hosted model without separate approval.

- **2.12 [ ] Decision — Continue investigations with approved partial evidence.**  
  Status: adopted by owner 2026-10-14; Answers 2.11.  
  When some ticket evidence is withheld for privacy review, continue the investigation with the independently approved information rather than pausing for approval. Report the withheld or unavailable evidence without revealing its contents, distinguish facts from hypotheses, and mark conclusions uncertain or unclear when the available evidence is insufficient. Withheld content never reaches the hosted investigator without separate approval; human review of that content remains possible.

- **2.3 [ ] Question — How stable should pseudonyms be?**  
  Status: open since 2026-10-14.  
  Case-scoped mappings reduce linkability; tenant-stable mappings ease cross-case analysis but need retention, deletion and collision policies.

- **2.10 [ ] Decision — Tenant-stable pseudonyms across cases.**  
  Status: adopted by owner 2026-10-14; Answers 2.3.  
  Within each tenant, the same identified person or other mapped entity keeps the same typed placeholder across separate support cases to support correlation. The trusted vault owns the mappings; identities and mappings must be isolated between tenants, without exposing raw values, lookup authority, or the vault to the hosted investigator. Stable pseudonyms remain personal-data risk rather than anonymization. Retention, rotation, deletion and tenant offboarding policies still require qualification before real-data use.

- **2.4 [ ] Question — Which ticket source and customer environment should the first real adapter serve?**  
  Status: open since 2026-10-14.  
  Jira versus Azure DevOps, exact permitted SQL views and logs, deployment constraints and real workflow acceptance remain unselected.

- **2.8 [ ] Decision — Azure DevOps is the first ticket source.**  
  Status: adopted by owner 2026-10-14; Answers 2.4 (ticket-source selection).  
  The first real TicketSource adapter targets Azure DevOps work items and their discussion history, using project-specific configuration and the trusted gateway. Keep Jira and other systems as future interchangeable adapters. This is a source selection only, not approval to access real work items or to write back to Azure DevOps; the initial investigation demonstration remains synthetic.

- **2.9 [ ] Question — What customer environment should the first Azure DevOps integration target?**  
  Status: open since 2026-10-14; remaining environment aspect of 2.4, not a blocker for synthetic demonstration.  
  Identify the Azure DevOps organization/project, authorized data/log sources, and reliable tenant/environment resolution fields before a real-customer adapter or deployment is scoped.

- **2.5 [ ] Question — What are the externally testable privacy/security claims?**  
  Status: open since 2026-10-14.  
  Dictionary/NER scanning misses unknown identities and contextual re-identification. Define residual-risk expectations, evaluation corpus, audit retention and breach handling before claims of safe disclosure.

## 3. Future expansion

- **3.1 [ ] Question — When should external reusable engines be adopted?**  
  Status: open since 2026-10-14, not an MVP blocker.  
  Reuse NLP Stack detection, Search Engine similarity, Media Intelligence evidence composition and optional Workflow Editor authoring only when a specific vertical slice proves necessary. The application owns tenant policy and support-case lifecycle.

## 4. Assessment delivery

- **4.1 [ ] Question — Default delivery of completed support assessments?**  
  Status: open since 2026-10-14; relevant after Azure DevOps source selection and evidence-backed reporting.  
  After an assessment passes trusted output checks, should Support Triage keep it in a private review workspace, automatically post an internal Azure DevOps comment, or prepare an internal comment draft requiring explicit approval? The choice concerns report delivery, not execution of suggested scripts. No report may bypass tenant permissions, the trusted output boundary or privacy safeguards. The first synthetic demonstration still requires human review.
