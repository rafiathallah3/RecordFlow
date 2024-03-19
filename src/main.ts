import { invoke } from "@tauri-apps/api/tauri";
import { Event, listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window';
import { open, save } from "@tauri-apps/api/dialog";

let ApakahRecording = false;
let ApakahLagiMain = false;
let JumlahDataInput = 0;

async function GantiStatusRecorder(status: boolean) {
	if(ApakahLagiMain) {
		return;
	}

	ApakahRecording = status;
	if(ApakahRecording) {
		TombolRecord.innerText = "Stop Recording (F6)";
		TombolRecord.classList.remove("btn-success");
		TombolRecord.classList.add("btn-danger");
		JumlahDataInput = 0;
		document.getElementById("TableInput")!.textContent = '';
		await appWindow.minimize();
	} else {
		TombolRecord.innerText = "Record (F6)";
		TombolRecord.classList.add("btn-success");
		TombolRecord.classList.remove("btn-danger");
	}
}

async function GantiStatusMainRecorder(status: boolean) {
	if(ApakahRecording) {
		return;
	}

	ApakahLagiMain = status;
	if(ApakahLagiMain) {
		TombolMainRecord.innerText = "Stop Playing (F7)";
		TombolMainRecord.classList.remove("btn-success");
		TombolMainRecord.classList.add("btn-danger");
		await appWindow.minimize();
	} else {
		TombolMainRecord.innerText = "Play (F7)";
		TombolMainRecord.classList.add("btn-success");
		TombolMainRecord.classList.remove("btn-danger");
		await appWindow.show();
	}
}

console.log("mulai");
await listen('KirimDataInput', (event: Event<{ tipe: string, value: string, waktu: number }>) => {
	if(!ApakahLagiMain) {
		JumlahDataInput++;
		const ElementInput = document.createElement('tr');
		ElementInput.innerHTML = `
			<th scope="row">${JumlahDataInput}</th>
			<td>${event.payload.tipe}</td>
			<td>(${event.payload.value})</td>
			<td>${event.payload.waktu}</td>
		`
		document.getElementById("TableInput")!.append(ElementInput)
		console.log(event.payload);
	}
});

await listen("StatusRecorder", (event: Event<boolean>) => {
	console.log("DAPAT", ApakahRecording);
	GantiStatusRecorder(event.payload);
});

await listen("SelesaiRecording", (event: Event<boolean>) => {
	GantiStatusMainRecorder(!event.payload);
});

await listen("DapatinSimpananFile", async (event: Event<boolean>) => {
	if(event.payload) {
		const savePath = await save({
			title: "Save Record Flow Data",
			filters: [{
				name: "Record Flow Data",
				extensions: ["rf"]
			}]
		});
		if(!savePath) return;
		await invoke("simpan_file", { path: savePath });
	};
});

await listen("BukaFile", async (event: Event<boolean>) => {
	if(event.payload) {
		const selected = await open({
			multiple: false,
			filters: [{
				name: "Record Flow Data",
				extensions: ["rf"]
			}]
		});

		await invoke("buka_file", { path: selected });
	}
});

const TombolRecord = document.getElementById("TombolRecord")!;
TombolRecord.onclick = async (e) => {
	e.preventDefault();
	GantiStatusRecorder(!ApakahRecording);

	await invoke("mulai_record");
}

const TombolMainRecord = document.getElementById("TombolMainRecord")!;
TombolMainRecord.onclick = async (e) => {
	e.preventDefault();
	if(ApakahRecording || JumlahDataInput <= 0) {
		return;
	}

	GantiStatusMainRecorder(true);

	await appWindow.minimize();
	await invoke("mainkan_recorder");
}