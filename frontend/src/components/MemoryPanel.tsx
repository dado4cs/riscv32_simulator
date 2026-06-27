import { useState } from "react";

interface MemoryPanelProps {
  onReadMemory: (address: number) => number | null;
}

export default function MemoryPanel({ onReadMemory }: MemoryPanelProps) {
  const [addressInput, setAddressInput] = useState("");
  const [result, setResult] = useState<number | null>(null);
  const [error, setError] = useState<string>("");

  const handleRead = () => {
    setError("");
    setResult(null);

    if (addressInput.trim() === "") return;

    const address = addressInput.startsWith("0x")
      ? parseInt(addressInput, 16)
      : parseInt(addressInput, 10);

    if (isNaN(address) || address < 0) {
      setError("Dirección inválida");
      return;
    }

    const value = onReadMemory(address);
    if (value === null) {
      setError("Out of bounds (>4095)");
    } else {
      setResult(value);
    }
  };

  return (
    <div className="bg-zinc-900 border border-zinc-600 rounded-3xl p-4 font-mono text-zinc-400 flex flex-col gap-3 shadow-2xs">
      <h3 className="text-zinc-500 uppercase text-xs font-bold tracking-wider">
        Memory Inspector (1 Byte)
      </h3>
      
      <div className="flex flex-row gap-2">
        <input
          type="text"
          placeholder="Ej: 4092 o 0xFFC"
          value={addressInput}
          onChange={(e) => setAddressInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleRead()}
          className="bg-black border border-zinc-700 rounded-lg px-3 py-1.5 text-zinc-300 w-full focus:outline-none focus:border-blue-500 transition-colors"
        />
        <button
          onClick={handleRead}
          className="bg-zinc-800 hover:bg-zinc-700 text-white px-4 py-1.5 rounded-lg border border-zinc-600 transition-colors"
        >
          Read
        </button>
      </div>

      <div className="h-8 flex items-center">
        {error && <span className="text-red-400 text-sm">{error}</span>}
        {result !== null && (
          <div className="flex gap-4 text-sm w-full bg-black p-2 rounded-lg border border-zinc-800 items-center justify-between">
            <span className="text-zinc-500">Valor en 0x{parseInt(addressInput).toString(16).toUpperCase()}:</span>
            <div className="flex gap-3 font-bold">
              <span className="text-emerald-400">0x{result.toString(16).toUpperCase().padStart(2, "0")}</span>
              <span className="text-zinc-400">({result})</span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
