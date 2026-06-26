import { useState } from "react";
import RegisterCard from "./RegisterCard";

const ABI_NAMES = [
  "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
  "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
  "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
  "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"
];

interface RegisterFileProps{
  registers: number[];
}


export default function RegisterFile({ registers }: RegisterFileProps) {
  const [signed, setSigned] = useState<boolean[]>(new Array(32).fill(true));

  const toggleSigned = (index: number) => {
    setSigned((prev ) => {
      const copy = [...prev];
      copy[index] = !copy[index];
      return copy;
    });
  };

  const setAllSigned = () => {
    setSigned(new Array(32).fill(true));
  };
  const setAllUnsigned = () => {
    setSigned(new Array(32).fill(false));
  };

  return (
    <div className="flex flex-col gap-4 bg-zinc-800 shadow-2xl rounded-3xl">
      <div className="flex flex-row items-center justify-between p-3 bg-zinc-900 border border-zinc-700 rounded-lg shadow-md">
        <h2 className="text-lg font-bold text-zinc-200 font-mono">Register File</h2>
        <div className="flex gap-3">
          <button
            onClick={setAllSigned}
            className="px-3 py-1 text-sm bg-gray-800 text-white font-mono rounded-3xl shadow hover:bg-indigo-600 transition-colors cursor-pointer"
          >
            All signed
          </button>
          <button
            onClick={setAllUnsigned}
            className="px-3 py-1 text-sm bg-gray-800 text-white font-mono rounded-3xl shadow hover:bg-indigo-600 transition-colors cursor-pointer"
          >
            All unsigned
          </button>
        </div>
      </div>    <div className="grid grid-rows-8 grid-flow-col gap-2 p-1.5">
      {registers.map((value, idx) => (
        <div key={idx}>
          <RegisterCard
            num={idx}
            abiName={ABI_NAMES[idx]}
            value={value}
            signed={signed[idx]}
            onToggle={() => toggleSigned(idx)}
          />
        </div>
      ))}
    </div>
    </div>
  );


}
