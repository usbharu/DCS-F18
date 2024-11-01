import { listen } from "@tauri-apps/api/event"
import { Data, Output } from "./Metrics"
import { useEffect, useState } from "react";

export const Metric: React.FC<{ output: Output }> = ({ output }) => {

    const [data, setData] = useState<number | string>();

    const mask = !!output.mask ? output.mask : 65535
    const shift = !!output.shift_by ? output.shift_by : 0

    function value(oldData: string | number | undefined, newData: string | number | undefined): string | number {
        if (output.type == "string") {
            if (newData === undefined) {
                return output.suffix + ""
            }
            let d = newData as string;    
            if (d.length > output.max_length) {
                d = d.substring(0, output.max_length);
            } else {
                let od = oldData as string | undefined;                
                d = d + od?.substring(od.length - (od.length - d.length), od.length)
            }
            return output.suffix + d
        }
        if (newData === undefined) {
            return 0
        }
        return ((newData as number) & mask) >> shift
    }

    useEffect(() => {
        let unlisten = listen("data", (d) => {

            let dat = d.payload as Data[];
            console.log(dat);
            
            dat.forEach(element => {
                if (element.address == output.address) {
                    setData(value(data, element.value))
                }
            });
        })

        return () => {
            unlisten.then((r) => {
                r()
            }).catch((e) => console.error(e))
        }
    }, [data])


    return (
        <span title={output.description}>Address: {output.address} Value: {data} Mask: {output.mask} Shift: {output.shift_by} Type: {output.type}</span>
    )
}