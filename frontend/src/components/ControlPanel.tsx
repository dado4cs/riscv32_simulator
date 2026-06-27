interface ControlPanelProps {
  pc: number;
  cpuReady: boolean;
  fileName: string;
  onStep: () => void;
  onRun: () => void;
  onReset: () => void;
  onLoadFile: () => void;
}

export default function ControlPanel({
  pc,
  cpuReady,
  fileName,
  onStep,
  onRun,
  onReset,
  onLoadFile,
}: ControlPanelProps) {
  return (
    <div className="bg-zinc-900 flex flex-row justify-between gap-1.5 p-1.5 rounded-2xl font-mono">
      <div className="w-100">
      <div className="font-mono text-white p-1.5 bg-gray-900 rounded-2xl  border border-blue-500 shadow w-70 flex flex-row justify-between">
        <span> PC: 0x{pc.toString(16).toUpperCase().padStart(8, "0")}</span>
        <span> ({pc})</span>
      </div>
      </div>
      <div className="text-white flex flex-row gap-3 items-center">
        <button
          className="bg-gray-900 hover:bg-gray-800 p-1.5 rounded-2xl border border-emerald-400 w-20"
          onClick={onStep}
          disabled={!cpuReady}
        >
          Step
        </button>

        <button
          className="bg-gray-900 hover:bg-gray-800 p-1.5 rounded-2xl border border-emerald-400 w-20"
          onClick={onRun}
          disabled={!cpuReady}
        >
          Run
        </button>
        <button
          className="bg-gray-900 hover:bg-gray-800 p-1.5 rounded-2xl border border-emerald-400 w-20"
          onClick={onReset}
          disabled={!cpuReady}
        >
          Reset
        </button>
      </div>
      <div className="flex flex-row justify-between gap-5 items-center w-100">
        <button
          className="bg-gray-900 hover:bg-gray-800 p-1.5 rounded-2xl border border-white text-white"
          onClick={onLoadFile}
          disabled={!cpuReady}
        >
          Load file (.bin)
        </button>
        <span className="text-gray-400 p-1.5">{fileName}</span>
      </div>
    </div>
  );
}
