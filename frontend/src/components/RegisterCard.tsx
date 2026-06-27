import { useEffect, useState } from "react";

interface RegisterCardProps {
  num: number;
  abiName: string;
  value: number;
  signed: boolean;
  onToggle: () => void;
}

export default function RegisterCard({
  num,
  abiName,
  value,
  signed,
  onToggle,
}: RegisterCardProps) {
  const displayValue = signed ? value | 0 : value;

  const [isFlashing, setIsFlashing] = useState(false);

  useEffect(() => {
    setIsFlashing(true);
    const timer = setTimeout(() => {
      setIsFlashing(false);
    }, 300);
  }, [value, num]);

  return (
    <div
      className={`flex flex-row justify-between items-center p-2 border rounded-2xl font-mono shadow-2xs text-white transition-all duration-300 ${
  isFlashing
    ? "scale-[1.02] bg-blue-900/80 border-blue-300 shadow-[0_0_15px_rgba(96,165,250,0.5)] z-10" 
    : num === 0
    ? "bg-zinc-800/50 border-blue-900 text-zinc-400" 
    : "bg-zinc-900/40 border-blue-500"
}`}    >
      <div className="flex flex-row gap-2 w-24">
        <span
          className={
            isFlashing
              ? "font-bold text-emerald-400"
              : "font-bold text-zinc-100"
          }
        >
          x{num}
        </span>
        <span className={isFlashing ? "text-emerald-200" : "text-zinc-500"}>
          ({abiName})
        </span>
      </div>
      <div className="flex flex-row gap-2">
        <span
          className={isFlashing ? "text-white font-bold" : "text-emerald-400"}
        >
          0x{value.toString(16).toUpperCase().padStart(8, "0")}
        </span>
        <span
          className={`ml-2 ${isFlashing ? "text-emerald-200" : "text-zinc-500"}`}
        >
          {" "}
          ({displayValue}){" "}
        </span>

        <div className="flex items-center gap-2 ml-4">
          <span className="text-xs text-blue-200">
            {signed ? "Sig" : "UnS"}
          </span>
          <button
            onClick={onToggle}
            className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none ${
              signed
                ? "bg-emerald-600 hover:bg-emerald-400"
                : "bg-emerald-800 hover:bg-emerald-700"
            }`}
          >
            <span
              className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
                signed ? "translate-x-5" : "translate-x-1"
              }`}
            />
          </button>
        </div>
      </div>
    </div>
  );
}
