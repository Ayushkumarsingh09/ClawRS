export type Agent = {
  id: string;
  name: string;
  kind: string;
  model: string;
  system_prompt: string;
  description: string;
};

export type Session = {
  id: string;
  agent_id: string;
  title: string;
  updated_at: string;
};

export type Message = {
  id: string;
  role: string;
  content: string;
  created_at: string;
};

export type Status = {
  version: string;
  provider: string;
  default_model: string;
  stats: { agents: number; sessions: number; messages: number };
};
