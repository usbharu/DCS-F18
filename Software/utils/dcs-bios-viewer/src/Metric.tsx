import { useEffect, useState } from "react";
import type { Data, Output } from "./Metrics";
import { listen } from "@tauri-apps/api/event";

export const Metric: React.FC<{ output: Output }> = ({ output }) => {
	const [data, setData] = useState<number | string>();

	function value(data: string | number | undefined): string | number {
		if (output.type === "string") {
			if (data === undefined) {
				return `${output.suffix}`;
			}
			return output.suffix + (data as string);
		}
		if (data === undefined) {
			return 0;
		}
		return data;
	}

	useEffect(() => {
		const unlisten = listen("data", (d) => {
			const dat = d.payload as Data[][];
      
			const b: Data[] | undefined = dat.find((p) => {
        
				if (output.type === "string") {
					return p.StringData.id === `${output.address}_string_${output.max_length}`;
				}
				return (
					p.IntegerData.id === `${output.address}_integer_${output.mask}_${output.shift_by}`
				);
			});
      
			if (b !== undefined) {
        if (output.type === "string") {
          setData(b.StringData.value);
        }else {
          setData(b.IntegerData.value);
        }
				
			}
		});

		return () => {
			unlisten
				.then((r) => {
					r();
				})
				.catch((e) => console.error(e));
		};
	});

	return (
		<span title={output.description}>
			Address: {output.address} Value: {value(data)} Mask: {output.mask} Shift:{" "}
			{output.shift_by} Type: {output.type}
		</span>
	);
};
