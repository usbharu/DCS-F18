import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export const MetricsView = () => {
	const [modules, setModules] = useState<string[]>([]);
	const [categories, setCategories] = useState<string[]>([]);
	const [ids, setIds] = useState<string[]>([]);

	const [selectedModule, setSelectedModule] = useState<string>();
	const [selectedCategory, setSelectedCategory] = useState<string>();
	const [selectedId, setSelectedId] = useState<string>();

	useEffect(() => {
		invoke("categories", { module: selectedModule })
			.then((e) => {
				setCategories(e);
			})
			.catch((e) => console.error(e));

		return () => {};
	}, [selectedModule]);

	useEffect(() => {
		invoke("ids", { module: selectedModule, category: selectedCategory })
			.then((e) => {
				setIds(e);
			})
			.catch((e) => console.error(e));

		return () => {};
	}, [selectedModule, selectedCategory]);

	useEffect(() => {
		invoke("modules")
			.then((e) => {
				setModules(e);
			})
			.catch((e) => console.error(e));

		return () => {};
	}, []);

	useEffect(() => {
		setSelectedModule(modules[0]);

		return () => {};
	}, [modules]);

	useEffect(() => {
		setSelectedCategory(categories[0]);

		return () => {};
	}, [categories]);

	useEffect(() => {
		setSelectedId(ids[0]);

		return () => {};
	}, [ids]);

	function onSubmit() {
		invoke("subscribe", { module: selectedModule, id: selectedId })
			.then((e) => console.log(e))
			.catch((e) => console.error(e));
	}

	return (
		<div>
			<form
				onSubmit={(e) => {
					e.preventDefault();
					onSubmit();
				}}
			>
				<div>
					<div className="select-metrics-select">
						<label htmlFor="select-metrics-module-select">Module</label>
						<select
							id="select-metrics-module-select"
							onSelect={(e) => setSelectedModule(e.currentTarget.value)}
						>
							{modules?.map((v) =>
								v === selectedModule ? (
									<option selected key={v}>
										{v}
									</option>
								) : (
									<option key={v}>{v}</option>
								),
							)}
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
					<div className="select-metrics-select">
						<input id="add" type="submit" value={"追加"} />
					</div>
				</div>
			</form>
		</div>
	);
};
