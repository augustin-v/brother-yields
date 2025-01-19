'use client';

import { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import ParticlesBackground from "./components/ParticlesBackground";
import PoolStats from './components/PoolStats';

export default function Home() {
  const [prompt, setPrompt] = useState('');
  const [messages, setMessages] = useState<Array<{content: string, isUser: boolean}>>([]);
  const [sessionId, setSessionId] = useState<string | null>(null);

  useEffect(() => {
    const initSession = async () => {
      try {
        const res = await fetch('http://localhost:5050/init-session');
        const data = await res.json();
        if (data.status === 'success') {
          setSessionId(data.message);
        }
      } catch (error) {
        console.error('Failed to initialize session:', error);
      }
    };

    initSession();
  }, []);

  const handleSubmit = async () => {
    if (!prompt.trim() || !sessionId) return;
    
    setMessages(prev => [...prev, { content: prompt, isUser: true }]);
    setPrompt('');

    try {
      const res = await fetch('http://localhost:5050/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ 
          prompt,
          sessionId 
        }),
      });
      
      const data = await res.json();
      if (data.status === 'success') {
        setMessages(prev => [...prev, { 
          content: data.message.replace(/^"|"$/g, ''), 
          isUser: false 
        }]);
      }
    } catch (error) {
      console.error('Error:', error);
    }
  };

  return (
    <div className="relative flex h-screen overflow-hidden">
      <ParticlesBackground />
      <PoolStats />
      
      {/* Main Chat Container */}
      <main className="flex-1 flex flex-col h-screen max-w-5xl mx-auto w-full">
        {/* Header */}
        <header className="flex flex-col items-center justify-center p-8 z-10">
          <h2 className="text-sm text-zinc-400 font-light">Brother Yield</h2>
          <h1 className="text-3xl font-bold text-white mt-2">
            Discuss the best Defi strategies on Starknet
          </h1>
        </header>

        {/* Messages Container */}
        <div className="flex-1 overflow-y-auto px-4">
          <div className="max-w-3xl mx-auto space-y-6 pb-24">
            {messages.map((message, index) => (
              <div
                key={index}
                className={`flex ${message.isUser ? 'justify-end' : 'justify-start'}`}
              >
                <div
                  className={`max-w-[85%] rounded-2xl p-4 ${
                    message.isUser
                      ? 'bg-purple-500/20 text-white'
                      : 'bg-zinc-900/80 backdrop-blur-sm border border-zinc-800/50 text-white'
                  }`}
                >
                  {message.isUser ? (
                    <div className="whitespace-pre-wrap">{message.content}</div>
                  ) : (
                    <ReactMarkdown 
                      className="prose prose-invert max-w-none prose-p:leading-relaxed prose-pre:bg-zinc-900"
                    >
                      {message.content}
                    </ReactMarkdown>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Input Container */}
        <div className="fixed bottom-0 left-0 right-0 bg-gradient-to-t from-black to-transparent p-4 z-50">
          <div className="max-w-3xl mx-auto">
            <div className="relative group">
              <textarea
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    handleSubmit();
                  }
                }}
                placeholder="Ask about Starknet DeFi strategies..."
                className="w-full bg-zinc-900/90 backdrop-blur-xl border border-zinc-800 rounded-xl px-6 py-4 pr-12 
                  focus:outline-none focus:ring-1 focus:ring-purple-500/30 focus:border-purple-500/30
                  placeholder-zinc-400 transition-all duration-200 ease-in-out resize-none overflow-hidden
                  min-h-[60px] max-h-[200px]"
                rows={1}
                style={{ height: 'auto' }}
              />
              <button 
                onClick={handleSubmit}
                className="absolute right-4 top-1/2 -translate-y-1/2 p-2 text-zinc-400 
                  hover:text-white transition-colors rounded-lg hover:bg-zinc-800/50"
              >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-5 h-5">
                  <path d="M3.478 2.404a.75.75 0 00-.926.941l2.432 7.905H13.5a.75.75 0 010 1.5H4.984l-2.432 7.905a.75.75 0 00.926.94 60.519 60.519 0 0018.445-8.986.75.75 0 000-1.218A60.517 60.517 0 003.478 2.404z" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
