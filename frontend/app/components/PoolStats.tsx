'use client';

import { useEffect, useState } from 'react';

interface Token {
  name: string;
  address: string;
  priceUSD: Price;  // Changed from number to Price
}
interface Price {
  integral: number;
  fractional: number;
  decimals: number;
}
interface PoolYield {
  token: Token;
  apy: number;
  tvl: number;
  volume_24h: number;
  risk_score: number;
  pool_type: 'Stable' | 'Degen';
}

interface PoolStats {
  yields: PoolYield[];
}

const priceToNumber = (price: Price): number => {
  return price.integral + (price.fractional / Math.pow(10, price.decimals));
};

export default function PoolStats() {
  const [poolStats, setPoolStats] = useState<PoolStats | null>(null);
  const [showDegen, setShowDegen] = useState(false);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const response = await fetch('http://127.0.0.1:5050/yields');
        const data = await response.json();
        setPoolStats(data);
      } catch (error) {
        console.error('Error fetching pool stats:', error);
      }
    };

    fetchStats();
    const interval = setInterval(fetchStats, 300000);
    return () => clearInterval(interval);
  }, []);

  const getWarnings = (pool: PoolYield) => {
    const warnings = [];
    if (pool.risk_score > 50) warnings.push('High Risk');
    if (pool.volume_24h < 10000) warnings.push('Low Volume');
    if (pool.tvl < 1000000) warnings.push('Low TVL');
    if (pool.apy > 500) warnings.push('Unusually High APY');
    return warnings;
  };

  const filteredPools = poolStats?.yields.filter(pool => 
    showDegen ? pool.pool_type === 'Degen' : pool.pool_type === 'Stable'
  );

  return (
    <div className="fixed right-4 top-24 w-80 space-y-4 z-50">
      <div className="flex justify-between items-center mb-4">
        <h2 className="font-bold">Estimated yields stats</h2>
        <button
          onClick={() => setShowDegen(!showDegen)}
          className={`px-3 py-1 rounded-lg text-sm ${
            showDegen
              ? 'bg-red-500 text-white'
              : 'bg-blue-500 text-white'
          }`}
        >
          {showDegen ? '🔥 Degen Pools' : '🏦 Standard Pools'}
        </button>
      </div>

      {filteredPools?.map((pool) => (
        <div
          key={`${pool.token.address}-${pool.pool_type}`}
          className="bg-zinc-900/50 backdrop-blur-xl border border-zinc-800 rounded-lg p-4"
        >
          {/* Rest of your pool display code remains the same */}
          <div className="flex justify-between items-center mb-2">
            <h3 className="text-lg font-bold">{pool.token.name}/USDC</h3>
            <span className="text-sm text-zinc-400">
              ${priceToNumber(pool.token.priceUSD).toFixed(2)}
            </span>
          </div>
          
          <div className="space-y-2">
            <div className="flex justify-between">
              <span>APY</span>
              <span className={pool.apy * 100 > 500 ? 'text-yellow-400' : 'text-green-400'}>
                {(pool.apy * 100).toFixed(2)}%
              </span>
            </div>
            <div className="flex justify-between">
              <span>TVL</span>
              <span>${pool.tvl.toLocaleString()}</span>
            </div>
            <div className="flex justify-between">
              <span>24h Volume</span>
              <span>${pool.volume_24h.toLocaleString()}</span>
            </div>
            <div className="flex justify-between">
              <span>Risk Score</span>
              <span className={pool.risk_score > 50 ? 'text-red-400' : 'text-green-400'}>
                {pool.risk_score.toFixed(2)}
              </span>
            </div>
          </div>

          {getWarnings(pool).length > 0 && (
            <div className="mt-2 pt-2 border-t border-zinc-800">
              {getWarnings(pool).map((warning, index) => (
                <div
                  key={index}
                  className="text-xs text-red-400 flex items-center gap-1"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    className="w-4 h-4"
                  >
                    <path
                      fillRule="evenodd"
                      d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z"
                      clipRule="evenodd"
                    />
                  </svg>
                  {warning}
                </div>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
