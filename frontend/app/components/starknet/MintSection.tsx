"use client";
import { useContract, useSendTransaction } from "@starknet-react/core";
import { Abi } from "starknet";

const abi = [
    {
        "type": "impl",
        "name": "BrotherYieldPassImpl",
        "interface_name": "contracts::contract::interface::IBrotherYieldPass"
    },
    {
        "type": "interface",
        "name": "contracts::contract::interface::IBrotherYieldPass",
        "items": [
            {
                "type": "function",
                "name": "mint",
                "inputs": [],
                "outputs": [],
                "state_mutability": "external"
            },
            {
                "type": "function",
                "name": "get_holders_num",
                "inputs": [],
                "outputs": [
                    {
                        "type": "core::integer::u32"
                    }
                ],
                "state_mutability": "view"
            },
            {
                "type": "function",
                "name": "check_minted",
                "inputs": [],
                "outputs": [],
                "state_mutability": "external"
            }
        ]
    },
] as const satisfies Abi;

export function MintSection({ onMintSuccess }: { onMintSuccess: () => void }) {
    const { contract } = useContract({
        abi,
        address: "0x03a4a729f942c231a9c95a25b5d9624fb1ae93e9db7ec98449e1ddff12437f38"
    });

    const { send: mint, isPending: mintPending, error: mintError } = useSendTransaction({
        calls: contract ? [
            contract.populate("mint", [])
        ] : undefined
    });

    const { send: checkMinted, isPending: checkPending, error: checkError } = useSendTransaction({
        calls: contract ? [
            contract.populate("check_minted", [])
        ] : undefined
    });

    const handleMint = async () => {
        try {
            await mint();
            onMintSuccess();
        } catch (error) {
            console.error("Mint failed:", error);
        }
    };

    const handleCheckMinted = async () => {
        try {
            await checkMinted();
            onMintSuccess();
        } catch (error) {
            console.error("Check minted failed:", error);
        }
    };

    return (
        <div className="flex flex-col items-center gap-4">
            <button
                onClick={handleMint}
                disabled={mintPending}
                className="px-8 py-4 bg-blue-500 text-white rounded disabled:bg-gray-400"
            >
                {mintPending ? 'Minting...' : 'Mint pass to register'}
            </button>

            <p className="text-lg font-medium text-white">Already minted?</p>
            
            <button
                onClick={handleCheckMinted}
                disabled={checkPending}
                className="px-8 py-4 bg-green-500 text-white rounded disabled:bg-gray-400"
            >
                {checkPending ? 'Checking...' : 'Login'}
            </button>
            
            {mintError && <p className="text-red-500">{mintError.message}</p>}
            {checkError && <p className="text-red-500">{checkError.message}</p>}
        </div>
    );
}