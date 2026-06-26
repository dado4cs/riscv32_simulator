interface Instructions{
  instructions : string[];
  current_pc : number;
}

export default function InstructionsPanel({instructions, current_pc} : Instructions){
    return (
    <div className="bg-zinc-900 shadow-2xs font-mono text-zinc-400 flex flex-1 flex-col gap-1.5 p-4 overflow-y-auto rounded-lg border border-zinc-600">
      {instructions.length === 0 ? (
        <span>Waiting for program</span>
      ) : (
        instructions.map((instruction, idx) => (
          <div key={idx}>
            <span
              className={
                idx === current_pc ? "bg-zinc-800" : "bg-zinc-900"
              }
            >
              {instruction}
            </span>
          </div>
        ))
      )}
    </div>
  );
}
