import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const View = () => {

    const [categories, setCategories] = useState<string[]>([])
    const [ids, setIds] = useState<string[]>([])
    const [selectedId, setSelectedId] = useState<string>()

    const [views, setViews] = useState<string[]>([])

    $: listen("source", (event) => {
        console.log(event);
    })

    useEffect(() => {
        invoke("category").then((v) => {
            setCategories(v as string[]);
            getIds(categories[0])
        })
    }, []);
    useEffect(() => {
        fetchSubscribed()
    }, []);

    function getIds(categoryName: string) {
        console.log("get-ids")
        invoke("ids", { categoryName: categoryName }).then((v) => {
            let v1 = v as string[];
            setIds(v1);
            setSelectedId(v1[0])
        })
    }

    function fetchSubscribed() {
        invoke("get_subscribed").then((v) => setViews(v as string[]))
    }

    function subscribe(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        invoke("subscribe", { id: selectedId }).then(() => {
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
                <select onChange={(e) => getIds(e.currentTarget.value)}>
                    {
                        categories.map(category => (
                            <option key={category} value={category}>{category}</option>
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
                    <p key={value}>{value} </p>
                ))
            }
        </div>
    )
}