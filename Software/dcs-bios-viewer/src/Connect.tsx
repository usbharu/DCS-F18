import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";

export const Connect = () => {

    const [bind, setBind] = useState<string>();
    const [address, setAddress] = useState<string>();
    const [interfaces, setInterfaces] = useState<string>();

    const navigate = useNavigate();

    function onSubmit() {
        invoke("connect", {
            bind: bind,
            address: address,
            interfae: interfaces
        }).then(() => navigate("/view")).catch((e) => console.error(e));

    }

    return (
        <div>
            <form onSubmit={(e) => {
                e.preventDefault();
                onSubmit()
            }}>
                <label>Bind<input onChange={(e) => setBind(e.currentTarget.value)}/></label>
                <label>Address<input onChange={(e) => setAddress(e.currentTarget.value)}/></label>
                <label>Interface<input onChange={(e) => setInterfaces(e.currentTarget.value)}/></label>
                <input type="submit"/>
            </form>
        </ div>
    );
}