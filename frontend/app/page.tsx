'use client';

import { useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';
import ParticlesBackground from "./components/ParticlesBackground";
import PoolStats from './components/PoolStats';
import { useAccount } from '@starknet-react/core';
import { WalletConnect } from './components/starknet/WalletConnect';
import { MintSection } from './components/starknet/MintSection';


export default function Home() {
  const [prompt, setPrompt] = useState('');
  const [messages, setMessages] = useState<Array<{content: string, isUser: boolean}>>([]);
  const { address, status } = useAccount();
  const [hasMinted, setHasMinted] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const API_URL = 'https://brother-yields.onrender.com';


  const initSession = async () => {
    if (!address) return;
    try {
        console.log("Initializing session for address:", address);
        const existingSessionId = localStorage.getItem('sessionId');
        
        if (existingSessionId) {
            console.log("Found existing session:", existingSessionId);
            const validateRes = await fetch(`${API_URL}/validate-session`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ session_id: existingSessionId }),
            });
            
            const validateData = await validateRes.json();
            if (validateRes.ok && validateData.status === 'success') {
                setSessionId(existingSessionId);
                return;
            }
            console.log("Session validation failed, creating new session");
            localStorage.removeItem('sessionId');
        }
        
        const res = await fetch(`${API_URL}/init-session`);
        if (!res.ok) {
            throw new Error(`Session initialization failed: ${res.status}`);
        }
        const data = await res.json();
        
        if (data.status === 'success') {
            console.log("New session created:", data.message);
            localStorage.setItem('sessionId', data.message);
            setSessionId(data.message);
        }
    } catch (error) {
        console.error('Failed to initialize session:', error);
        localStorage.removeItem('sessionId');
    }
};


  useEffect(() => {
    if (hasMinted) {
      initSession();
    }
  }, [hasMinted]);

  const handleSubmit = async () => {
    if (!prompt.trim() || !sessionId || isLoading) return;
    
    setIsLoading(true);
    setMessages(prev => [...prev, { content: prompt, isUser: true }]);
    setPrompt('');

    try {
      const res = await fetch(`${API_URL}/prompt`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ 
          prompt,
          session_id: sessionId 
        }),
      });
      
      const contentType = res.headers.get('content-type');
      if (!contentType || !contentType.includes('application/json')) {
        throw new Error('Server returned non-JSON response');
      }

      const data = await res.json();
      if (data.status === 'success') {
        setMessages(prev => [...prev, { 
          content: data.message.replace(/^"|"$/g, ''), 
          isUser: false 
        }]);
      } else {
        // Handle session invalidation
        if (data.message.includes('Session not found')) {
          localStorage.removeItem('sessionId');
          await initSession(); // Re-initialize session instead of page reload
        }
      }
    } catch (error) {
      console.error('Error:', error);
      setMessages(prev => [...prev, { 
        content: 'Sorry, something went wrong. Please try again.', 
        isUser: false 
      }]);
      // Try to reinitialize session on error
      await initSession();
    } finally {
      setIsLoading(false);
    }
  };
  
  const handleMintSuccess = () => {
    setHasMinted(true);
  };

  return (
    <div className="relative flex h-screen overflow-hidden">
      <ParticlesBackground />
      <PoolStats />
      
      <main className="flex-1 flex flex-col h-screen max-w-5xl mx-auto w-full">
        <header className="flex flex-col items-center justify-center p-8 z-10">
          <h2 className="text-sm text-zinc-400 font-light">Brother Yield</h2>
          <h1 className="text-3xl font-bold text-white mt-2">
            Discuss the best Defi strategies on Starknet
          </h1>
        </header>
  
        {!address ? (
          <WalletConnect />
        ) : !hasMinted ? (
          <MintSection onMintSuccess={handleMintSuccess} />
        ) : !sessionId ? (
          <div className="text-white text-center">Initializing session...</div>
        ) : (
          <>
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
          </>
        )}
      </main>
    </div>
  );
  
}
