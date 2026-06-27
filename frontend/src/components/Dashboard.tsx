import { useEffect, useRef, useState } from "react";
import init, { Cpu } from "risc_v_simulator";
import RegisterFile from "./RegisterFile";
import Console from "./Console";
import ControlPanel from "./ControlPanel";
import InstructionsPanel from "./InstructionsPanel";

export default function Dashboard() {
  const [cpuReady, SetCpuReady] = useState<boolean>(false);
  const [registers, setRegisters] = useState(new Array(32).fill(0));
  const [logs, setLogs] = useState<string[]>([]);
  const [pc, setPc] = useState(0);
  const cpuRef = useRef<Cpu | null>(null);
  const [fileName, setFileName] = useState("No file loaded");
  const [instructions, setInstructions] = useState<string[]>([]);
  const [lastInstructions, setLastInstructions] = useState<Uint8Array>(new Uint8Array)

  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const initCpu = async () => {
      await init();
      const cpu = new Cpu(1024 * 4);
      cpuRef.current = cpu;
      const initialRegs = new Array(32)
        .fill(0)
        .map((_, i) => cpu.get_register(i));

      setRegisters(initialRegs);
      setLogs(["[System]: Risc-V-Simulator Ready"]);
      SetCpuReady(true);
    };
    initCpu();
  }, []);

  const onStep = () => {
    const possibleLog = cpuRef.current?.step();
    setPc(cpuRef.current?.pc ?? 0);
    const regs = new Array(32)
      .fill(0)
      .map((_, i) => cpuRef.current?.get_register(i) ?? 0);

    setRegisters(regs);

    if(possibleLog){
      setLogs((prev)=> [...prev, `[ECALL] ${possibleLog}`]);
    }
  };

  const onRun = () => {};

  const hardReset = () => {
    const newCpu = new Cpu(1024 * 4);
    cpuRef.current = newCpu;
    setPc(newCpu.pc);
    const regs = new Array(32)
      .fill(0)
      .map((_, i) => cpuRef.current?.get_register(i));
    setRegisters(regs);

    setInstructions([]);
    setLogs(["[System]: Risc-V-Simulator restarted"])

  };

  const onReset = () => {
    const newCpu = new Cpu(1024 * 4);
    newCpu.load_program(lastInstructions);
    cpuRef.current = newCpu;
    setPc(newCpu.pc);
    const regs = new Array(32)
      .fill(0)
      .map((_, i) => cpuRef.current?.get_register(i));
    setRegisters(regs);

    setLogs(["[System]: Risc-V-Simulator restarted"])
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file || !cpuRef.current) return;

    setFileName(file.name);
    hardReset();

    const reader = new FileReader();
    reader.onload = (event) => {
      const bytes = new Uint8Array(event.target?.result as ArrayBuffer).slice();
      cpuRef.current?.load_program(bytes);
      setLastInstructions(bytes);

      const allInstructions = cpuRef.current?.disassemble_all();

      if(allInstructions) {
        setInstructions(allInstructions);
      }

      setLogs((prev) => [
        ...prev,
        `[System]: '${file.name}' loaded (${bytes.length} bytes).`,
      ]);
    };
    reader.readAsArrayBuffer(file);
  };


  return (
    <>
      <input
        type="file"
        ref={fileInputRef}
        onChange={handleFileChange}
        className="hidden"
        accept=".bin"
      />
      <div className="flex h-screen flex-col justify-between bg-gray-950 p-3">
        <ControlPanel
          pc={pc}
          cpuReady={cpuReady}
          fileName={fileName}
          onStep={() => {
            onStep();
          }}
          onRun={() => {
            onRun();
          }}
          onReset={() => onReset()}
          onLoadFile={() => fileInputRef.current?.click()}
        />
        <div className="flex flex-row flex-1 min-h-0 gap-3 p-3">
        <RegisterFile registers={registers} />
          <InstructionsPanel instructions={instructions} current_pc={pc}/>
        </div>
        <Console logs={logs} />
      </div>
    </>
  );
}
