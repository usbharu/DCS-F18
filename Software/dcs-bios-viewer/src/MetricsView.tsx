import { useState } from "react";

export const MetricsView = () => {
	const [modules, setModules] = useState<string[]>();
	const [categories, setCategories] = useState<string[]>();
	const [ids, setIds] = useState<string[]>();

	return (
		<div>
			<form>
				<div>
					<div className="select-metrics-select">
						<label htmlFor="select-metrics-module-select">Module</label>
						<select id="select-metrics-module-select">
							{modules?.map((v) => (
								<option key={v}>{v}</option>
							))}
						</select>
					</div>
					<div className="select-metrics-select">
						<label htmlFor="select-metrics-category-select">Category</label>
						<select id="select-metrics-category-select">
							{categories?.map((v) => (
								<option key={v}>{v}</option>
							))}
						</select>
					</div>
					<div className="select-metrics-select">
						<label htmlFor="select-metrics-id-select">ID</label>
						<select id="select-metrics-id-select">
							{ids?.map((v) => (
								<option key={v}>{v}</option>
							))}
						</select>
					</div>
                    <button type="submit">追加</button>
				</div>
			</form>
		</div>
	);
};
