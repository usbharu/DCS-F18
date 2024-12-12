import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Metrics, type Function } from "./Metrics";

export const MetricsView = () => {
	const [modules, setModules] = useState<string[]>([]);
	const [categories, setCategories] = useState<string[]>([]);
	const [ids, setIds] = useState<string[]>([]);

	const [selectedModule, setSelectedModule] = useState<string>();
	const [selectedCategory, setSelectedCategory] = useState<string>();
	const [selectedId, setSelectedId] = useState<string>();

	const [views, setViews] = useState<Function[]>([]);

	useEffect(() => {

		invoke("categories", { moduleName: selectedModule })
			.then((e) => {
				setCategories(e);
			})
			.catch((e) => console.error(e));

		return () => {};
	}, [selectedModule]);

	useEffect(() => {
		invoke("ids", {
			moduleName: selectedModule,
			categoryName: selectedCategory,
		})
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
		invoke("subscribe", { moduleName: selectedModule, id: selectedId })
			.then((e) => invoke("get_subscribed").then((v) => setViews(v as Function[])))
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
							onChange={(e) => setSelectedModule(e.currentTarget.value)}
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
						<select
							id="select-metrics-category-select"
							onChange={(e) => setSelectedCategory(e.currentTarget.value)}
						>
							{categories?.map((v) => (
								<option key={v}>{v}</option>
							))}
						</select>
					</div>
					<div className="select-metrics-select">
						<label htmlFor="select-metrics-id-select">ID</label>
						<select
							id="select-metrics-id-select"
							onChange={(e) => setSelectedId(e.currentTarget.value)}
						>
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
			<div>
				{views.map((value) => (
					<Metrics
						key={value.identifier}
						fun={value}
						OnRemove={(v) =>
							invoke("get_subscribed").then((v) => setViews(v as Function[]))
						}
					/>
				))}
			</div>
		</div>
	);
};
