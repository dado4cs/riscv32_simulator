interface ConsoleLogs {
  logs: string[];
}

export default function Console({ logs }: ConsoleLogs) {
  return (
    <div className="bg-black shadow-2xs h-40 font-mono text-zinc-400 flex flex-col gap-1.5 p-4 overflow-y-auto rounded-lg border border-zinc-600">
      {logs.length === 0 ? (
        <span>Waiting for program</span>
      ) : (
        logs.map((log, idx) => (
          <div key={idx}>
            <span className="text-green-400 font-bold">{"> "}</span>
            <span
              className={
                log.includes("Exit code") ? "text-indigo-400" : "text-zinc-400"
              }
            >
              {log}
            </span>
          </div>
        ))
      )}
    </div>
  );
}
