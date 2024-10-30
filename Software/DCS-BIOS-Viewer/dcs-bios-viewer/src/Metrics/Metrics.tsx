import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react"

export type Output = {
    address: number,
    description: string,
    mask?: number,
    max_value?: number,
    shift_by?: number,
    suffix: string,
    type: string
}

export type Function = {
    api_variant?: string,
    category: string,
    control_type: string,
    description: string,
    identifier: string,
    momentary_positions?: string,
    outputs: Output[]
}

export type Data = {
    address: number,
    value: number
}

export type OnRemove = (id: string) => void;

export const Metrics: React.FC<{ fun: Function, OnRemove: OnRemove }> = ({ fun, OnRemove }) => {

    const [data, setData] = useState<Data[]>([])
    const addressList = fun.outputs.map((o) => o.address);

    useEffect(() => {
        console.log(data);

    }, [data])


    useEffect(() => {
        let unlisten = listen("data", (d) => {
            console.log(d);
            let dat = d.payload as Data[]
            let b: Data[] = dat.filter((p) => addressList.includes(p.address));
            if (b.length != 0) {
                setData(b)
            }
        })

        return () => {
            unlisten.then((result) => {
                result();
            }).catch((err) => {
                console.error(err);

            });
        }
    }, [fun])

    function remove(e: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        e.preventDefault();
        invoke("unsubscribe",{id:fun.identifier}).then(() => {
            OnRemove(fun.identifier)
        }).catch((e)=>console.error(e)
        );
    }

    return (
        <div>
            <p>{fun.identifier}</p>
            {
                data.sort((a, b) => a.address - b.address).map((v) => (
                    <p key={v.address}>{v.address} + {v.value}</p>
                )
                )
            }
            <button onClick={(e) => remove(e)}>Remove</button>
        </div>
    )
}