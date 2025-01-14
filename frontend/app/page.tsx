'use client';

import { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import ParticlesBackground from "./components/ParticlesBackground";
import PoolStats from './components/PoolStats';

export default function Home() {
  const [prompt, setPrompt] = useState('');
  const [messages, setMessages] = useState<Array<{content: string, isUser: boolean}>>([]);

  const handleSubmit = async () => {
    if (!prompt.trim()) return;
    
    // Add user message
    setMessages(prev => [...prev, { content: prompt, isUser: true }]);
    setPrompt('');

    try {
      const res = await fetch('http://localhost:5050/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ prompt }),
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
<div className="relative min-h-screen flex flex-col">
  <ParticlesBackground />
  <PoolStats />
  <div className="space-y-4 text-center p-4 z-10">
    <h2 className="text-sm text-zinc-400 font-light">
    Brother Yield
    </h2>
    <h1 className="text-4xl font-bold text-white">
      Discuss the best Defi strategies on Starknet.
    </h1>
  </div>

  <div className="flex-1 overflow-y-auto px-4 pb-24">
    <div className="w-full max-w-2xl mx-auto space-y-4">
      {messages.map((message, index) => (
        <div
          key={index}
          className={`flex ${message.isUser ? 'justify-end' : 'justify-start'}`}
        >
          <div
            className={`max-w-[80%] rounded-lg p-4 ${
              message.isUser
                ? 'bg-purple-500/20 text-white'
                : 'bg-zinc-900/50 backdrop-blur-xl border border-zinc-800 text-white'
            }`}
          >
            {message.isUser ? (
              message.content
            ) : (
              <ReactMarkdown className="prose prose-invert">
                {message.content}
              </ReactMarkdown>
            )}
          </div>
        </div>
      ))}
    </div>
  </div>

  <div className="fixed bottom-20 left-0 right-0 px-4 z-50">
    <div className="max-w-2xl mx-auto relative group">
      <input
        type="text"
        value={prompt}
        onChange={(e) => setPrompt(e.target.value)}
        placeholder="How can I help you?"
        className="w-full bg-zinc-900/50 backdrop-blur-xl border border-zinc-800 rounded-lg px-6 py-4 pr-12 text-lg
        focus:outline-none focus:ring-1 focus:ring-purple-500/10 focus:border-purple-500/20
        placeholder-zinc-400 transition-all duration-300 ease-in-out"
        onKeyDown={(e) => e.key === 'Enter' && handleSubmit()}
      />
      <button 
        onClick={handleSubmit}
        className="absolute right-4 top-1/2 -translate-y-1/2 text-zinc-400 hover:text-white transition-colors"
      >
      </button>
    </div>
  </div>

</div>

  );
}
