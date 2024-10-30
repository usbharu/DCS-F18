import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Function, Metrics } from "../Metrics/Metrics";

export const View = () => {

    const [modules, setModules] = useState<string[]>([])
    const [categories, setCategories] = useState<string[]>([])
    const [ids, setids] = useState<string[]>([])
    const [selectedId, setSelectedId] = useState<string>()
    const [selectedModule, setSetselectedModule] = useState<string>()

    const [views, setViews] = useState<Function[]>([])

    useEffect(() => {
        invoke("modules").then((v) => {
            setModules(v as string[]);
            getCategories(v[0])
        })
    },[]);

    useEffect(() => {
        fetchSubscribed()
    }, []);

    useEffect(()=>{
        setSetselectedModule(modules[0])
    },[modules])

    useEffect(()=>{
        getIds(selectedModule,categories[0])
    },[categories])

    useEffect(()=>{
        setSelectedId(ids[0])
    },[ids])

    function getCategories(moduleName: string) {
        setSetselectedModule(moduleName)
        invoke("categories", { moduleName: moduleName }).then((v) => {
            let v1 = v as string[];
            setCategories(v1);

        }).catch((err)=>{
            console.error(err);
            
        })
    }

    function getIds(moduleName: string | undefined, categoryName: string) {
        if (moduleName === undefined) {
            return;
        }
        invoke("ids", { moduleName: moduleName, categoryName: categoryName }).then((v) => {
            setids(v as string[]);
        }).catch((err) => {
            console.error(err);

        });
    }

    function fetchSubscribed() {
        invoke("get_subscribed").then((v) => setViews(v as Function[]))
    }

    function subscribe(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        invoke("subscribe", { moduleName:selectedModule,id: selectedId }).then(() => {
            fetchSubscribed()
        }).catch((err) => {
            console.error(err);

        });
    }

    return (
        <div>
            <form onSubmit={(e) => {
                subscribe(e)
            }}>
                <select onChange={(e) => getCategories(e.currentTarget.value)}>
                    {
                        modules.map(module => (
                            <option key={module} value={module}>{module}</option>
                        ))
                    }
                </select>
                <select onChange={(e) => getIds(selectedModule, e.currentTarget.value)}>
                    {
                        categories.map(id => (
                            <option key={id} value={id}>{id}</option>
                        ))
                    }
                </select>
                <select onChange={(e) => setSelectedId(e.currentTarget.value)}>
                    {
                        ids.map(id => (
                            <option key={id} value={id}>{id}</option>
                        ))
                    }
                </select>
                <input type="submit" />
            </form>
            {
                views.map(value => (
                    <Metrics key={value.identifier} fun={value} OnRemove={(v)=>fetchSubscribed()}></Metrics>
                ))
            }
        </div>
    )
}