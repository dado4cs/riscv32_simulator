interface Instructions {
  instructions: string[];
  current_pc: number;
}

export default function InstructionsPanel({ instructions, current_pc }: Instructions) {
  const activeIdx = current_pc / 4;

  return (
    <div className="bg-zinc-900 shadow-2xs font-mono text-zinc-400 flex flex-1 flex-col gap-1.5 p-4 overflow-y-auto rounded-3xl border border-zinc-600">
      {instructions.length === 0 ? (
        <span className="text-zinc-500 italic">Waiting for program...</span>
      ) : (
        instructions.map((instruction, idx) => {
          const isCurrent = idx === activeIdx;

          return (
            <div
              key={idx}
              className={`flex flex-row items-center gap-2 px-2 py-1 rounded-lg transition-all duration-150 ${
                isCurrent
                  ? "bg-blue-950/50 border border-blue-500/30 text-blue-400 font-bold shadow-[0_0_10px_rgba(59,130,246,0.1)]"
                  : "bg-transparent border border-transparent text-zinc-400"
              }`}
            >
              <span className={`text-xs w-3 ${isCurrent ? "text-blue-400" : "text-transparent"}`}>
                ➔
              </span>
              <span>{instruction}</span>
            </div>
          );
        })
      )}
    </div>
  );
}
