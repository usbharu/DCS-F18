import { invoke } from "@tauri-apps/api/core";
import { Metric } from "./Metric";

export type Output = {
	address: number;
	description: string;
	mask?: number;
	max_value?: number;
	shift_by?: number;
	suffix: string;
	type: string;
  max_length?: number;
};

export type Function = {
	api_variant?: string;
	category: string;
	control_type: string;
	description: string;
	identifier: string;
	momentary_positions?: string;
	outputs: Output[];
};

export type Data = {
  id: string,
	address: number;
	value: number | string;
};

export type OnRemove = (id: string) => void;

export const Metrics: React.FC<{ fun: Function; OnRemove: OnRemove }> = ({
	fun,
	OnRemove,
}) => {

  function remove(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    e.preventDefault();
    invoke("unsubscribe", { id: fun.identifier }).then(() => {
        OnRemove(fun.identifier)
    }).catch((e) => console.error(e)
    );
}

	return (
		<div className="metrics">
			<button className="metrics-remove-button" type="button" onClick={(e) => remove(e)}>
				x
			</button>
			<div className="metrics-holder">
				<div>
					<h4 className="metrics-description">{fun.description}</h4>
					<span className="metrics-id">{fun.identifier}</span>
				</div>
				{fun.outputs
					.sort((a, b) => a.address - b.address)
					.map((v) => (
						<Metric key={v.address} output={v} />
					))}
			</div>
		</div>
	);
};
