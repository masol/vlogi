**English** | [中文](docs/README.cn.md)


# vlogi.cc

**vlogi.cc** is an all-in-one **multi-agent automation and prompt orchestration platform**.  
It combines a flexible **visual workflow editor** with **agent-based execution**, enabling both **usage** and **creation** of prompt-driven logic chains.

---

## 🚀 What is vlogi.cc?

vlogi.cc is built around the **Input / Output / Process** model.  
Instead of writing long, rigid prompts, you design structured AI tasks called **PromptFlows** — reusable, composable building blocks that define how agents interact and pass data.

Each PromptFlow can:

- Define specific inputs and expected outputs.
- Contain one or more AI “agents” that process or generate content.
- Be nested or connected to other PromptFlows.

---

## 🧠 How it Works

1. **Structured Prompts** → Prompts are represented as data structures (like JSON).  
2. **Recursive Composition** → One prompt can generate parts of another, enabling self-assembling multi-agent systems.  
3. **No Context Engineering Required** → Agents share relevant information automatically via structured definitions.  
4. **Visual Editing Interface** → A clear, n8n-style UI lets you build, debug, and visualize your PromptFlows.  
5. **User Context Integration** → The system can use and store each user’s context for personalized automation.

---

## 🔗 PromptFlow Example

A simple PromptFlow might take a “blog topic” as input, generate an outline, then call another agent to write paragraphs for each section — all defined visually:

```
Blog Topic → Outline Generator Agent → Section Writer Agent → Markdown Output
```

Each block (agent) can be reused in other PromptFlows.

---

## 🌍 Collaboration & Reuse

- **Explore & Download:** Use PromptFlows shared by others to complete common tasks.  
- **Edit & Create:** Modify or build new PromptFlows with the drag-and-drop visual editor.  
- **Share & Scale:** Publish your PromptFlows for the community to use and remix.

---

## ⚙️ Key Features

- 🔁 Multi-agent orchestration without prompt engineering overhead  
- 🧩 Modular, composable **PromptFlows**  
- 🪶 Lightweight JSON-based prompt structure  
- 🖥️ Visual workflow builder (similar to n8n)  
- 🧍 Personalized context-aware agent execution  
- 🌐 Community-driven sharing and versioning  

---

## 🛠️ Future Directions

- Integrations with external APIs and tools  
- Agent marketplace & version control  
- Real-time collaborative editing  
- Configurable runtime environments  

---

## 📄 License

MIT License © 2025 vlogi.cc contributors

---

**vlogi.cc** — Build, reuse, and evolve AI workflows with structure, recursion, and collaboration.
