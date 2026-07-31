import { useCallback, useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { api, type Agent, type Message, type Session, type Status } from "./api/client";
import styles from "./App.module.css";

export default function App() {
  const [status, setStatus] = useState<Status | null>(null);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [agent, setAgent] = useState<Agent | null>(null);
  const [sessions, setSessions] = useState<Session[]>([]);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  const loadCore = useCallback(async () => {
    const [st, ag] = await Promise.all([api.status(), api.agents()]);
    setStatus(st);
    setAgents(ag);
    if (!agent && ag.length > 0) setAgent(ag[0]);
  }, [agent]);

  useEffect(() => {
    loadCore().catch((e) => setError(String(e)));
  }, [loadCore]);

  useEffect(() => {
    if (!agent) return;
    api.sessions(agent.id).then(setSessions).catch((e) => setError(String(e)));
  }, [agent]);

  useEffect(() => {
    if (!sessionId) {
      setMessages([]);
      return;
    }
    api.messages(sessionId).then(setMessages).catch((e) => setError(String(e)));
  }, [sessionId]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, sending]);

  async function newSession() {
    if (!agent) return;
    const s = await api.createSession(agent.id);
    setSessions((prev) => [s, ...prev]);
    setSessionId(s.id);
    setMessages([]);
  }

  async function send() {
    if (!sessionId || !input.trim() || sending) return;
    const text = input.trim();
    setInput("");
    setSending(true);
    setError(null);
    const optimistic: Message = {
      id: crypto.randomUUID(),
      role: "user",
      content: text,
      created_at: new Date().toISOString(),
    };
    setMessages((m) => [...m, optimistic]);
    try {
      const res = await api.chat(sessionId, text);
      setMessages((m) => [
        ...m,
        {
          id: crypto.randomUUID(),
          role: "assistant",
          content: res.reply,
          created_at: new Date().toISOString(),
        },
      ]);
      const refreshed = await api.sessions(agent!.id);
      setSessions(refreshed);
    } catch (e) {
      setError(String(e));
    } finally {
      setSending(false);
    }
  }

  return (
    <div className={styles.shell}>
      <header className={styles.header}>
        <div className={styles.brand}>
          <img src="/logo.png" alt="" className={styles.logo} />
          <div>
            <div className={styles.title}>ClawRS</div>
            <div className={styles.tagline}>Rust-native agent platform</div>
          </div>
        </div>
        <div className={styles.headerMeta}>
          {status && (
            <>
              <span className={styles.pill}>
                <span className={styles.dot} />
                {status.provider}
              </span>
              <span className={styles.metaText}>{status.default_model}</span>
            </>
          )}
        </div>
      </header>

      <div className={styles.body}>
        <aside className={styles.sidebar}>
          <div className={styles.sidebarSection}>
            <div className={styles.sectionLabel}>Agents</div>
            <ul className={styles.agentList}>
              {agents.map((a) => (
                <li key={a.id}>
                  <button
                    type="button"
                    className={
                      agent?.id === a.id ? styles.agentActive : styles.agentBtn
                    }
                    onClick={() => {
                      setAgent(a);
                      setSessionId(null);
                    }}
                  >
                    <span className={styles.agentName}>{a.name}</span>
                    <span className={styles.agentModel}>{a.model}</span>
                  </button>
                </li>
              ))}
            </ul>
          </div>

          <div className={styles.sidebarSection}>
            <div className={styles.sectionRow}>
              <span className={styles.sectionLabel}>Sessions</span>
              <button type="button" className={styles.newBtn} onClick={newSession}>
                New
              </button>
            </div>
            <ul className={styles.sessionList}>
              {sessions.map((s) => (
                <li key={s.id}>
                  <button
                    type="button"
                    className={
                      sessionId === s.id ? styles.sessionActive : styles.sessionBtn
                    }
                    onClick={() => setSessionId(s.id)}
                  >
                    {s.title}
                  </button>
                </li>
              ))}
              {sessions.length === 0 && (
                <li className={styles.emptyHint}>Start a new chat</li>
              )}
            </ul>
          </div>

          {status && (
            <div className={styles.stats}>
              <div>
                <strong>{status.stats.messages}</strong>
                <span>messages</span>
              </div>
              <div>
                <strong>{status.stats.sessions}</strong>
                <span>sessions</span>
              </div>
            </div>
          )}
        </aside>

        <main className={styles.main}>
          {!sessionId ? (
            <div className={styles.welcome}>
              <h1>Run agents at wire speed</h1>
              <p>
                ClawRS orchestrates models, tools, and memory in a single Rust binary.
                Pick an agent, open a session, and send a message.
              </p>
              <button type="button" className={styles.primaryBtn} onClick={newSession}>
                Start session
              </button>
            </div>
          ) : (
            <>
              <div className={styles.messages}>
                {messages.map((m) => (
                  <article
                    key={m.id}
                    className={
                      m.role === "user" ? styles.msgUser : styles.msgAssistant
                    }
                  >
                    <div className={styles.msgRole}>
                      {m.role === "user" ? "You" : agent?.name ?? "Agent"}
                    </div>
                    <div className={styles.msgBody}>
                      {m.role === "assistant" ? (
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>
                          {m.content}
                        </ReactMarkdown>
                      ) : (
                        m.content
                      )}
                    </div>
                  </article>
                ))}
                {sending && (
                  <div className={styles.typing}>
                    <span />
                    <span />
                    <span />
                  </div>
                )}
                <div ref={bottomRef} />
              </div>

              <div className={styles.composer}>
                {error && <div className={styles.error}>{error}</div>}
                <div className={styles.composerRow}>
                  <textarea
                    rows={1}
                    placeholder="Message…"
                    value={input}
                    onChange={(e) => setInput(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        send();
                      }
                    }}
                  />
                  <button
                    type="button"
                    className={styles.sendBtn}
                    disabled={sending || !input.trim()}
                    onClick={send}
                  >
                    Send
                  </button>
                </div>
              </div>
            </>
          )}
        </main>

        <aside className={styles.inspector}>
          {agent && (
            <>
              <div className={styles.sectionLabel}>Agent</div>
              <h2 className={styles.inspectorTitle}>{agent.name}</h2>
              <dl className={styles.dl}>
                <dt>Kind</dt>
                <dd>{agent.kind}</dd>
                <dt>Model</dt>
                <dd className={styles.mono}>{agent.model}</dd>
              </dl>
              <div className={styles.sectionLabel}>System</div>
              <p className={styles.promptPreview}>{agent.system_prompt}</p>
            </>
          )}
        </aside>
      </div>
    </div>
  );
}
