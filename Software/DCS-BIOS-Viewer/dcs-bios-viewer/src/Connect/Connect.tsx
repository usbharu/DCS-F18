import { useNavigate } from "react-router-dom";
import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import React from "react";

export const Connect = () => {

    const [bind, setBind] = useState("0.0.0.0:5010")
    const [address, setAddress] = useState("239.255.50.10")
    const [interfaceAddr, setInterfaceAddr] = useState("0.0.0.0")
    const [errorMessage,setErrorMessage] = useState<string>()

    const history = useNavigate();

    function tryConnect(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        invoke("setup_socket", { bind: bind, addr: address, interface: interfaceAddr }).then(() => { history("/view") }).catch((e)=>{console.error(e);
            setErrorMessage(e)
        })
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
                {
                    !!errorMessage ? <p>{errorMessage}</p> : <React.Fragment/>
                }
                
                <div>
                    <input id={"submit-connect-input-form"} type="submit" />
                </div>
            </form>
        </div>
    )
}