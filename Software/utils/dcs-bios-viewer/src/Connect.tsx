import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";

export const Connect = () => {
	const [bind, setBind] = useState<string>("0.0.0.0:5010");
	const [address, setAddress] = useState<string>("239.255.50.10");
	const [interfaces, setInterfaces] = useState<string>("0.0.0.0");
	const [errorMessage, setErrorMessage] = useState("");

	const navigate = useNavigate();

	function onSubmit() {
		invoke("connect", {
			bind: bind,
			address: address,
			interface: interfaces,
		})
			.then(() => navigate("/view"))
			.catch((e) => { console.error(e); setErrorMessage(e) });
	}

	return (
		<div>
			<form
				onSubmit={(e) => {
					e.preventDefault();
					onSubmit();
				}}
			>
				<div id="connect-form">
					<div className="connect-form-bind connect-form-input">
						<label htmlFor="connect-form-bind-input">Bind </label>
						<input
							id="connect-form-bind-input"
							onChange={(e) => setBind(e.currentTarget.value)}
							defaultValue={bind}
						/>
					</div>
					<div className="connect-form-address  connect-form-input">
						<label htmlFor="connect-form-address-input">Address</label>
						<input
							id="connect-form-address-input"
							onChange={(e) => setAddress(e.currentTarget.value)}
							defaultValue={address}
						/>
					</div>
					<div className="connect-form-input connect-form-input">
						<label htmlFor="connect-form-interface-input">Interface</label>
						<input
							id="connect-form-interface-input"
							onChange={(e) => setInterfaces(e.currentTarget.value)}
							defaultValue={interfaces}
						/>
					</div>
					<div className="connect-form-input">
						<div />
						<input className="connect-form-submit" type="submit" value="接続" />
					</div>
					<p>{errorMessage}</p>
				</div>
			</form>
		</div>
	);
};
