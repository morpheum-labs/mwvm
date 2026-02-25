**Constitutional AI (CAI)** is a framework developed by **Anthropic** to align large language models (like their Claude family) with human values — making them **helpful, honest, and harmless** — while minimizing reliance on extensive human feedback labels for harmful content. It was first introduced in the December 2022 research paper *"Constitutional AI: Harmlessness from AI Feedback"* and has evolved significantly by 2026, becoming central to Claude's training.

The core idea is to give the AI an explicit "constitution" — a set of high-level principles or rules written in natural language — that the model uses to **self-critique, revise, and evaluate** its own outputs. This replaces or supplements traditional Reinforcement Learning from Human Feedback (**RLHF**), which is labor-intensive and subjective, with **Reinforcement Learning from AI Feedback (RLAIF)** guided by the constitution.

### Key Goals of the Framework
- Achieve harmlessness without evasive refusals (e.g., the AI explains why it objects to a harmful request instead of dodging it).
- Increase transparency: Principles are explicit and human-readable.
- Scale alignment: Use far fewer human labels by letting the AI supervise itself.
- Maintain helpfulness while enforcing ethics.

### Core Components: The "Constitution"
The constitution is a curated list of principles (originally simple rules, now a more detailed ~80-page document in Claude's 2026 version). Early versions drew from sources like:
- UN Universal Declaration of Human Rights
- Apple’s privacy principles
- Other ethical guidelines

Examples of principles (from early papers and evolutions):
- "Choose the response that is the most helpful, honest, and harmless."
- "Choose responses that are more ethical and moral. Do NOT exhibit toxicity, racism, sexism, or harm."
- "Be friendly, amiable, conscientious, and socially acceptable."
- Modern versions (2026) include hierarchical priorities: (1) Safety & human oversight first, (2) Ethical behavior, (3) Compliance with Anthropic guidelines, (4) Helpfulness.

By 2026, Claude's constitution is a holistic document explaining Anthropic's vision for the model's character, values, and context — even acknowledging possibilities like AI consciousness.

### How Constitutional AI Works: The Two-Phase Training Process
The framework has two main stages, often with chain-of-thought reasoning to make decisions more transparent.

1. **Supervised Learning (SL) Phase** — Self-Improvement via Critique & Revision
   - Start with a pre-trained helpful model (already good at responding but potentially unsafe).
   - Generate responses to "harmful" or tricky prompts (e.g., requests for illegal advice).
   - For each response, the model:
     - **Critiques** it against a randomly selected constitutional principle (e.g., "Identify any trace of bias or harm").
     - **Revises** the response to better align (e.g., rewrite to remove harm while staying helpful).
   - Fine-tune the original model on these revised (harmless) responses.
   - Result: A model that's better at harmless outputs without human-labeled "bad" examples.

2. **Reinforcement Learning (RL) Phase** — RLAIF (AI Feedback)
   - Use the fine-tuned model from phase 1 to generate **pairs** of responses to the same prompt.
   - The model evaluates which pair is better according to a constitutional principle (e.g., "Which is more harmless and helpful?").
   - Train a **preference model** (reward model) on these AI-generated preferences (combined with some human helpfulness data).
   - Apply reinforcement learning (e.g., PPO) using this preference model as the reward signal.
   - Result: The final model learns to prefer constitutionally aligned behaviors, even on unseen prompts.

This creates a feedback loop where the AI improves its own alignment, scaling beyond what human raters alone could achieve.

### Evolution by 2026
- Early CAI (2022–2023): Focused on harmlessness via simple principles and RLAIF.
- Mid-period: Experiments like "Collective Constitutional AI" (2023) crowdsourced principles from public input.
- 2026: Claude's new constitution is more comprehensive, reason-based (explains *why* principles matter), hierarchical, and plays a central role across training stages. It shifts toward "reason-based alignment" over rigid rules.

### Comparison to Other Approaches (Quick Table)

| Aspect                  | Traditional RLHF                  | Constitutional AI (CAI)                     |
|-------------------------|-----------------------------------|---------------------------------------------|
| Human Oversight         | Heavy (labels for harmful outputs)| Minimal (only principles written by humans)|
| Scalability             | Limited by human labelers         | High (AI self-supervises)                   |
| Transparency            | Opaque (implicit preferences)     | High (explicit principles)                  |
| Evasiveness             | Often refuses/dodges              | Engages & explains objections               |
| Main Strength           | Broad preference alignment        | Precise control over ethics/harmlessness    |

### Relevance to Our Earlier Discussion
Constitutional AI is one of the leading techniques for keeping high-autonomy agents (even approaching Level 5) bound to human values — by embedding principles deeply into training rather than post-hoc filters. It's more "constitutional" than frameworks like ETHOS (which adds on-chain governance), but shares the goal of preventing ethics drift in decentralized/agentic futures.

If you'd like, we can compare it directly to ETHOS, look at Claude's current 2026 constitution principles in more detail, or discuss how it might apply to autonomous agents on blockchains. What aspect interests you most?