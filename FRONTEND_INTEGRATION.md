# 🔗 Intégration Frontend Next.js

Guide complet pour connecter le frontend Next.js au backend Rust.

---

## 📦 Installation des Dépendances

```bash
cd client

# Installer les dépendances nécessaires
npm install socket.io-client axios zustand
# ou
pnpm add socket.io-client axios zustand
```

### Packages utilisés

- **socket.io-client** - Client WebSocket
- **axios** - Client HTTP pour l'API REST
- **zustand** - State management simple et efficace

---

## 🏗️ Structure Recommandée

```
client/
├── app/
│   ├── (auth)/
│   │   ├── login/
│   │   └── signup/
│   ├── (chat)/
│   │   ├── servers/
│   │   └── channels/
│   └── layout.tsx
├── lib/
│   ├── api.ts              # Client API REST
│   ├── socket.ts           # Client WebSocket
│   └── types.ts            # Types TypeScript
├── stores/
│   ├── authStore.ts        # Store authentification
│   ├── chatStore.ts        # Store messages/canaux
│   └── socketStore.ts      # Store WebSocket
└── components/
    ├── chat/
    │   ├── MessageList.tsx
    │   ├── MessageInput.tsx
    │   └── TypingIndicator.tsx
    └── sidebar/
        ├── ServerList.tsx
        └── ChannelList.tsx
```

---

## 🔧 Configuration de Base

### 1. Variables d'Environnement

Créer `.env.local` :

```env
NEXT_PUBLIC_API_URL=http://localhost:3000
NEXT_PUBLIC_WS_URL=http://localhost:3000
```

### 2. Types TypeScript

`lib/types.ts` :

```typescript
export interface User {
  id: number;
  username: string;
  email: string;
  created_at: string;
}

export interface AuthResponse {
  user: User;
  token: string;
}

export interface Server {
  id: number;
  name: string;
  invitation_code: string;
  owner_id: number;
  created_at: string;
}

export interface Channel {
  id: number;
  name: string;
  channel_type: string;
  server_id: number;
  created_at: string;
}

export interface Message {
  id: number;
  content: string;
  channel_id: number;
  author_id: number;
  author_username: string;
  is_deleted: boolean;
  created_at: string;
  updated_at: string | null;
}

export interface UserRole {
  OWNER: 'OWNER';
  ADMIN: 'ADMIN';
  MEMBER: 'MEMBER';
}

// Événements WebSocket
export type SocketEvent =
  | { type: 'message:new'; data: MessageNewEvent }
  | { type: 'message:deleted'; data: MessageDeletedEvent }
  | { type: 'user:typing'; data: UserTypingEvent }
  | { type: 'user:connected'; data: UserConnectedEvent }
  | { type: 'user:disconnected'; data: UserDisconnectedEvent };

export interface MessageNewEvent {
  channel_id: number;
  message_id: number;
  content: string;
  author_id: number;
  author_username: string;
  created_at: string;
}

export interface MessageDeletedEvent {
  channel_id: number;
  message_id: number;
}

export interface UserTypingEvent {
  channel_id: number;
  user_id: number;
  username: string;
}

export interface UserConnectedEvent {
  server_id: number;
  user_id: number;
  username: string;
}

export interface UserDisconnectedEvent {
  server_id: number;
  user_id: number;
  username: string;
}
```

---

## 🌐 Client API REST

`lib/api.ts` :

```typescript
import axios from 'axios';
import type { AuthResponse, Server, Channel, Message, User } from './types';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

// Créer une instance axios
const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Intercepteur pour ajouter le token automatiquement
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Authentication
export const auth = {
  signup: async (username: string, email: string, password: string): Promise<AuthResponse> => {
    const { data } = await api.post<AuthResponse>('/auth/signup', { username, email, password });
    return data;
  },

  login: async (username: string, password: string): Promise<AuthResponse> => {
    const { data } = await api.post<AuthResponse>('/auth/login', { username, password });
    return data;
  },

  me: async (): Promise<User> => {
    const { data } = await api.get<User>('/auth/me');
    return data;
  },
};

// Servers
export const servers = {
  create: async (name: string): Promise<Server> => {
    const { data } = await api.post<Server>('/servers', { name });
    return data;
  },

  list: async (): Promise<Server[]> => {
    const { data } = await api.get<Server[]>('/servers');
    return data;
  },

  get: async (id: number): Promise<Server> => {
    const { data } = await api.get<Server>(`/servers/${id}`);
    return data;
  },

  join: async (invitationCode: string): Promise<Server> => {
    const { data } = await api.post<Server>('/servers/join', { invitation_code: invitationCode });
    return data;
  },

  leave: async (id: number): Promise<void> => {
    await api.delete(`/servers/${id}/leave`);
  },
};

// Channels
export const channels = {
  create: async (serverId: number, name: string): Promise<Channel> => {
    const { data } = await api.post<Channel>(`/servers/${serverId}/channels`, { name });
    return data;
  },

  list: async (serverId: number): Promise<Channel[]> => {
    const { data } = await api.get<Channel[]>(`/servers/${serverId}/channels`);
    return data;
  },

  get: async (id: number): Promise<Channel> => {
    const { data } = await api.get<Channel>(`/channels/${id}`);
    return data;
  },

  delete: async (id: number): Promise<void> => {
    await api.delete(`/channels/${id}`);
  },
};

// Messages
export const messages = {
  send: async (channelId: number, content: string): Promise<Message> => {
    const { data } = await api.post<Message>(`/channels/${channelId}/messages`, { content });
    return data;
  },

  list: async (channelId: number, limit?: number, offset?: number): Promise<Message[]> => {
    const { data } = await api.get<Message[]>(`/channels/${channelId}/messages`, {
      params: { limit, offset },
    });
    return data;
  },

  delete: async (id: number): Promise<void> => {
    await api.delete(`/messages/${id}`);
  },
};

export default api;
```

---

## 🔌 Client WebSocket

`lib/socket.ts` :

```typescript
import { io, Socket } from 'socket.io-client';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'http://localhost:3000';

class SocketClient {
  private socket: Socket | null = null;
  private token: string | null = null;

  connect(token: string) {
    this.token = token;

    this.socket = io(WS_URL, {
      transports: ['websocket', 'polling'],
      autoConnect: true,
    });

    this.socket.on('connect', () => {
      console.log('✓ Connecté au WebSocket');
      this.authenticate();
    });

    this.socket.on('disconnect', () => {
      console.log('✗ Déconnecté du WebSocket');
    });

    this.socket.on('error', (error: any) => {
      console.error('Erreur WebSocket:', error);
    });

    return this.socket;
  }

  private authenticate() {
    if (this.socket && this.token) {
      this.socket.emit('authenticate', { token: this.token });
    }
  }

  joinServer(serverId: number) {
    if (this.socket) {
      this.socket.emit('join_server', { server_id: serverId });
    }
  }

  leaveServer(serverId: number) {
    if (this.socket) {
      this.socket.emit('leave_server', { server_id: serverId });
    }
  }

  startTyping(channelId: number) {
    if (this.socket) {
      this.socket.emit('typing_start', { channel_id: channelId });
    }
  }

  stopTyping(channelId: number) {
    if (this.socket) {
      this.socket.emit('typing_stop', { channel_id: channelId });
    }
  }

  on(event: string, callback: (...args: any[]) => void) {
    if (this.socket) {
      this.socket.on(event, callback);
    }
  }

  off(event: string, callback?: (...args: any[]) => void) {
    if (this.socket) {
      this.socket.off(event, callback);
    }
  }

  disconnect() {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
  }
}

export const socketClient = new SocketClient();
export default socketClient;
```

---

## 📦 Store Zustand

### Auth Store

`stores/authStore.ts` :

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { User } from '@/lib/types';
import { auth } from '@/lib/api';
import { socketClient } from '@/lib/socket';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;

  login: (username: string, password: string) => Promise<void>;
  signup: (username: string, email: string, password: string) => Promise<void>;
  logout: () => void;
  loadUser: () => Promise<void>;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      token: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      login: async (username, password) => {
        set({ isLoading: true, error: null });
        try {
          const response = await auth.login(username, password);
          set({
            user: response.user,
            token: response.token,
            isAuthenticated: true,
            isLoading: false,
          });
          localStorage.setItem('token', response.token);
          socketClient.connect(response.token);
        } catch (error: any) {
          set({
            error: error.response?.data?.error || 'Erreur de connexion',
            isLoading: false,
          });
          throw error;
        }
      },

      signup: async (username, email, password) => {
        set({ isLoading: true, error: null });
        try {
          const response = await auth.signup(username, email, password);
          set({
            user: response.user,
            token: response.token,
            isAuthenticated: true,
            isLoading: false,
          });
          localStorage.setItem('token', response.token);
          socketClient.connect(response.token);
        } catch (error: any) {
          set({
            error: error.response?.data?.error || 'Erreur d\'inscription',
            isLoading: false,
          });
          throw error;
        }
      },

      logout: () => {
        localStorage.removeItem('token');
        socketClient.disconnect();
        set({
          user: null,
          token: null,
          isAuthenticated: false,
        });
      },

      loadUser: async () => {
        const token = localStorage.getItem('token');
        if (!token) return;

        try {
          const user = await auth.me();
          set({ user, token, isAuthenticated: true });
          socketClient.connect(token);
        } catch (error) {
          set({ user: null, token: null, isAuthenticated: false });
          localStorage.removeItem('token');
        }
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        token: state.token,
        user: state.user,
      }),
    }
  )
);
```

---

## 🎨 Composants Exemple

### Message List

`components/chat/MessageList.tsx` :

```typescript
'use client';

import { useEffect, useRef } from 'react';
import { useChatStore } from '@/stores/chatStore';

export default function MessageList({ channelId }: { channelId: number }) {
  const messages = useChatStore((state) => state.messages[channelId] || []);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="flex-1 overflow-y-auto p-4 space-y-4">
      {messages.map((message) => (
        <div key={message.id} className="flex items-start space-x-3">
          <div className="flex-1">
            <div className="flex items-center space-x-2">
              <span className="font-semibold">{message.author_username}</span>
              <span className="text-xs text-gray-500">
                {new Date(message.created_at).toLocaleTimeString()}
              </span>
            </div>
            <p className="text-gray-300">{message.content}</p>
          </div>
        </div>
      ))}
      <div ref={messagesEndRef} />
    </div>
  );
}
```

---

## 🚀 Prochaines Étapes

1. ✅ Créer les pages d'authentification (`/login`, `/signup`)
2. ✅ Créer la page principale du chat
3. ✅ Implémenter la liste des serveurs
4. ✅ Implémenter la liste des canaux
5. ✅ Implémenter l'envoi de messages
6. ✅ Connecter les événements WebSocket
7. ✅ Ajouter les notifications en temps réel

**Votre frontend est prêt à communiquer avec le backend ! 🎉**
