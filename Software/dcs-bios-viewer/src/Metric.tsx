import { useEffect, useState } from "react";
import type { Data, Output } from "./Metrics";
import { listen } from "@tauri-apps/api/event";

export const Metric: React.FC<{ output: Output }> = ({ output }) => {
	const [data, setData] = useState<number | string>();

	const mask = !!output.mask ? output.mask : 65535;
	const shift = !!output.shift_by ? output.shift_by : 0;

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
		return ((data as number) & mask) >> shift;
	}

	useEffect(() => {
		const unlisten = listen("data", (d) => {
			const dat = d.payload as Data[];
			const b: Data | undefined = dat.find((p) => p.address === output.address);
			if (b !== undefined) {
				setData(b.value);
			}
		});

		return () => {
			unlisten
				.then((r) => {
					r();
				})
				.catch((e) => console.error(e));
		};
	}, [output.address]);

	return (
		<span title={output.description}>
			Address: {output.address} Value: {value(data)} Mask: {output.mask} Shift:{" "}
			{output.shift_by} Type: {output.type}
		</span>
	);
};
