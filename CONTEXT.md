# Domain context

- **Support case:** a ticket, its comments and the evidence available to investigate it. Ticket-system identities are not automatically safe to export.
- **Clear/trusted zone:** holds customer-origin data, credentials, authorization, policy, vault and delivery.
- **Investigator zone:** runs analysis with limited, mediated tool access. Its output is untrusted until checked.
- **Gateway:** mandatory policy boundary for each information flow to the investigator, not merely a one-off preprocessing step.
- **Vault:** trusted mapping from clear values to typed pseudonyms. Identity scope, retention and coverage remain product questions.
- **Agent bundle:** only the values approved by a gateway policy for the investigator. Current Rust reference supports *exact approved literals* and *exact pre-seeded token values*, not arbitrary text.
- **Evidence reference:** a resolvable reference to an actual tool result, log span or code location. Reported diagnoses must distinguish supported causes from guesses.
- **Egress check:** independently checks investigator-bound and outbound content. A seeded dictionary can only find values known to it.
- **Human review:** approval of relevant disclosure and of proposed remediation. No generated script runs automatically.

The repository is distinct from [Workflow Editor](https://github.com/moritzbrantner/workflow-editor), [NLP Stack](https://github.com/moritzbrantner/nlp-stack), [Search Engine](https://github.com/moritzbrantner/search-engine), and [Media Intelligence](https://github.com/moritzbrantner/media-intelligence). Its boundary is the support-investigation workflow, not generic engines.
