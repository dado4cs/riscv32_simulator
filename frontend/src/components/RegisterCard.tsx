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

  return (
    <div
      className={`flex flex-row justify-between items-center p-2 border rounded-2xl border-blue-500 font-mono shadow-2xs text-white ${num === 0 ? "bg-zinc-800/50" : "bg-zinc-900/40"}`}
    >
      <div className="flex flex-row gap-2 w-24">
        <span className="font-bold">x{num}</span>
        <span>({abiName})</span>
      </div>
      <div className="flex flex-row gap-2">
        <span className="font-bold">
          0x{value.toString(16).toUpperCase().padStart(8, "0")} ({displayValue})
        </span>

        <div className="flex items-center gap-2 ml-4">
          <span className="text-xs text-blue-200">
            {signed ? "Sig" : "UnS"}
          </span>
          <button
            onClick={onToggle}
            className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none ${
              signed ? "bg-emerald-600 hover:bg-emerald-400" : "bg-emerald-800 hover:bg-emerald-700"
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
