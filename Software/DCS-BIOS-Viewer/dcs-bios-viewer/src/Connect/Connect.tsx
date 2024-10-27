import { useNavigate } from "react-router-dom";
import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export const Connect = () => {

    const [bind, setBind] = useState("0.0.0.0:5010")
    const [address, setAddress] = useState("255.239.50.10")
    const [interfaceAddr, setInterfaceAddr] = useState("0.0.0.0")

    const history = useNavigate();

    function tryConnect(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        invoke("setup_socket", { bind: bind, addr: address, interface: interfaceAddr }).then(() => { history("/view") })
    }

    return (
        <div>
            <form id={"connect-input-form"} onSubmit={(e) => tryConnect(e)}>
                <div className={"input-holder"}>
                    <label htmlFor={"input-bind"}>Bind</label>
                    <input id={"input-bind"} value={bind} onChange={(e) => setBind(e.target.value)} />
                </div>
                <div className={"input-holder"}>
                    <label htmlFor={"input-address"}>Address</label>
                    <input id={"input-address"} value={address} onChange={(e) => setAddress(e.target.value)} />
                </div>
                <div className={"input-holder"}>
                    <label htmlFor={"input-interfaceAddr"}>Interface Addr</label>

                    <input id={"input-interfaceAddr"} value={interfaceAddr}
                        onChange={(e) => setInterfaceAddr(e.target.value)} />
                </div>
                <div>
                    <input id={"submit-connect-input-form"} type="submit" />
                </div>
            </form>
        </div>
    )
}