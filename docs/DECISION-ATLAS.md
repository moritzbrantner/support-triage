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

- **2.13 [ ] Question — Where should the first trusted gateway run?**  
  Status: open since 2026-10-14; not a blocker for the synthetic demonstration.  
  For the first real-data deployment with a hosted AI model (Decision 2.6), should credentials, raw ticket/database/log data, pseudonym vault and policy gateway operate inside the customer's infrastructure, or in a managed Support Triage service permitted to access that raw data? This decides operational ownership and the external data-custody boundary; neither option relaxes gateway checks or authorizes production-data access by itself.

- **2.14 [ ] Decision — Customer-hosted trusted gateway and data custody.**  
  Status: adopted by owner 2026-10-14; Answers 2.13.  
  For the first real deployment, operate the trusted gateway/clear zone within the customer's infrastructure. Raw ticket, database and log evidence, credentials, tenant authorization, reversible pseudonym vault, re-identification and customer-side audit remain in that controlled environment; do not give a centrally managed Support Triage service custody of these values. The external hosted model (Decision 2.6) may receive only information explicitly approved by the gateway's validated release policy, including follow-up tool results. This fixes the trusted-zone location, not the investigator runtime placement, and does not by itself authorize production-data processing or establish a no-leak guarantee.

- **2.15 [ ] Question — Where should the investigator process run when the gateway is customer-hosted?**  
  Status: open since 2026-10-14; follows Decisions 2.6 and 2.14, not a blocker for the synthetic demonstration.  
  Should the constrained agent/investigator runtime run alongside the trusted gateway inside the customer's infrastructure and call the external model API from there, or run in a managed Support Triage environment with tightly mediated access to the customer-hosted gateway? The model remains hosted in either design. This determines operational burden and whether another externally operated runtime ever handles gateway-approved evidence.

- **2.16 [ ] Decision — Customer-hosted isolated investigator with external model inference.**  
  Status: adopted by owner 2026-10-15; Answers 2.15.  
  The constrained agent/investigator process runs inside the customer's infrastructure alongside, but isolated from, the customer-hosted trusted gateway (Decision 2.14). It obtains case evidence and follow-up tool results only through that gateway, with no direct access to raw customer data, source credentials or the pseudonym vault. The investigator calls an external hosted AI model (Decision 2.6) with only gateway-approved transformed content. There is no centrally managed Support Triage agent runtime receiving case evidence. This settles runtime placement, not model provider/account ownership, production deployment hardening or authorization to process real customer data.

- **2.17 [ ] Question — Who owns hosted-model API access and billing?**  
  Status: open since 2026-10-15; follows Decisions 2.6, 2.14 and 2.16; not a blocker for the synthetic demonstration.  
  Should each customer configure their own hosted-model provider account/API key and pay the provider directly, or should Support Triage provide centrally managed model access/billing while keeping the trusted gateway and investigator process customer-hosted? Both paths still send only policy-approved data to the hosted provider, but differ in onboarding, billing, operational responsibility and another potential processing intermediary.

## 3. Future expansion

- **3.1 [ ] Question — When should external reusable engines be adopted?**  
  Status: open since 2026-10-14, not an MVP blocker.  
  Reuse NLP Stack detection, Search Engine similarity, Media Intelligence evidence composition and optional Workflow Editor authoring only when a specific vertical slice proves necessary. The application owns tenant policy and support-case lifecycle.

## 4. Assessment delivery

- **4.1 [ ] Question — Default delivery of completed support assessments?**  
  Status: open since 2026-10-14; relevant after Azure DevOps source selection and evidence-backed reporting.  
  After an assessment passes trusted output checks, should Support Triage keep it in a private review workspace, automatically post an internal Azure DevOps comment, or prepare an internal comment draft requiring explicit approval? The choice concerns report delivery, not execution of suggested scripts. No report may bypass tenant permissions, the trusted output boundary or privacy safeguards. The first synthetic demonstration still requires human review.

- **4.2 [ ] Decision — Human-approved draft for Azure DevOps delivery.**  
  Status: adopted by owner 2026-10-14; Answers 4.1.  
  After the evidence-backed assessment passes trusted output checks, prepare an editable **internal-use Azure DevOps work-item comment draft** for the support engineer. Only an explicit human approval may publish the approved content to the work item; declining or leaving the draft pending must not post anything. Verify the actual target audience and permissions before posting rather than assuming Azure DevOps provides per-comment internal visibility. Suggested remediation scripts remain inert and require separate human execution.

- **4.3 [ ] Question — How should internal-only delivery work if the Azure DevOps work item is visible beyond support staff?**  
  Status: open since 2026-10-14; related to Decision 4.2, not a blocker for the synthetic demonstration.  
  The documented Azure DevOps work-item comment API has no per-comment internal/private visibility option. Should the reviewed draft remain inside Support Triage when the work item's audience is broader than intended, or be published to a separate access-restricted Azure DevOps work item? In either case, never disclose an internal draft to unintended recipients.

- **4.4 [ ] Decision — Keep internal assessments in Support Triage for broadly visible work items.**  
  Status: adopted by owner 2026-10-14; Answers 4.3.  
  If the Azure DevOps work item is visible outside the authorized support audience, keep its internal assessment and reviewed comment draft within an access-controlled Support Triage workspace; never publish the internal diagnosis to that work item or create a separate restricted Azure DevOps work item as a workaround. Explicit human approval alone does not override the audience boundary. Decision 4.2 still permits approved posting when target access has been independently verified as appropriately restricted; absent or uncertain verification, keep the draft private.

## 5. Investigation initiation

- **5.1 [ ] Question — What starts unattended investigation?**  
  Status: open since 2026-10-14; not a blocker for the synthetic demonstration.  
  In the first operational Azure DevOps integration, should unattended investigations start only for work items explicitly tagged/marked for triage, or for every new work item in a designated, authorized support queue? This determines workload, cost and exposure of ticket contents to gateway processing. Each project profile may configure its permitted source scope, and all investigations retain the same privacy boundary.

- **5.2 [ ] Decision — Require an explicit triage marker to start unattended investigation.**  
  Status: adopted by owner 2026-10-14; Answers 5.1.  
  In the initial Azure DevOps integration, automatically investigate only work items deliberately marked for triage (for example an `auto-triage` tag or a project-configured equivalent field) within an authorized project scope. New or updated work items without that marker do not start unattended investigations. The marker selects candidates; it does not bypass tenant authorization, privacy-gateway approval or other safety checks. The synthetic demonstration must not connect to a real Azure DevOps project.

- **5.3 [ ] Question — Should marked tickets be investigated again after changes?**  
  Status: open since 2026-10-14; follows Decision 5.2.  
  After a marked ticket has been investigated, should new comments or materially changed fields automatically trigger a fresh investigation while the marker remains present, or require an explicit retriage action? The first choice keeps diagnoses current but increases processing, and the second makes reruns deliberate but may leave stale drafts. In both cases, guard against duplicate event delivery and prevent an outdated assessment from being approved for the wrong ticket revision.

- **5.4 [ ] Decision — Automatically reinvestigate marked tickets after relevant changes.**  
  Status: adopted by owner 2026-10-14; Answers 5.3.  
  While a work item retains its explicitly configured triage marker within authorized scope, new comments or materially changed case information automatically initiate a fresh investigation. Unchanged, irrelevant, or repeatedly delivered events must not create duplicate investigations. Tie findings and review drafts to the source revision: if the ticket changes after assessment, an earlier draft is stale and cannot be approved or posted as current without renewed review of an up-to-date assessment. All reruns pass through the same tenant authorization, privacy gateway, and bounded investigation controls.
