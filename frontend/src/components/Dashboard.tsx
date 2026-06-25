import { useEffect, useRef, useState } from "react";
import init, { Cpu } from "risc_v_simulator";
import RegisterFile from "./RegisterFile";
import Console from "./Console";

export default function Dashboard() {
  const [cpuReady, SetCpuReady] = useState<boolean>(false);
  const [registers, setRegisters] = useState(new Array(32).fill(0));
  const [logs, setLogs] = useState<string[]>([]);
  const cpuRef = useRef<Cpu | null>(null);

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

  return (
    <div className="flex h-screen flex-col justify-between bg-gray-950 p-1.5">
      <RegisterFile registers={registers} />
      <Console logs={logs} />
    </div>
  );
}
