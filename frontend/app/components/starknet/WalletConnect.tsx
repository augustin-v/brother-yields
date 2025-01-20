"use client";
import { useConnect, useAccount, useInjectedConnectors } from "@starknet-react/core";
import { argent, braavos } from "@starknet-react/core";
import { useEffect } from "react";
export function WalletConnect() {
  const { connect } = useConnect();
  const { address, status } = useAccount();
  const { connectors } = useInjectedConnectors({
    recommended: [argent(), braavos()],
    includeRecommended: "onlyIfNoConnectors",
  });

  // Add status monitoring
  useEffect(() => {
    if (status === "connected") {
      console.log("Wallet connected successfully:", address);
    } else if (status === "disconnected") {
      console.log("Wallet disconnected");
    } else if (status === "connecting") {
      console.log("Connecting to wallet...");
    }
  }, [status, address]);

  return (
    <div className="flex flex-col items-center gap-4 mt-8">
      <div className="text-sm text-zinc-400 mb-2">
        Status: {status}
      </div>
      {connectors.map((connector) => (
        <button
          key={connector.id}
          onClick={() => {
            console.log(`Attempting to connect ${connector.id}...`);
            connect({ connector });
          }}
          className="px-6 py-3 bg-purple-500/20 rounded-xl hover:bg-purple-500/30 transition-colors"
        >
          Connect {connector.id}
        </button>
      ))}
    </div>
  );
}
