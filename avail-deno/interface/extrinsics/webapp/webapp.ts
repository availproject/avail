import { Application, Router, Status } from "https://deno.land/x/oak/mod.ts";
import { SDK } from "../../../src/sdk.ts";
import { SubmitDataView } from "./submit_data.ts";
import { SharedState } from "./shared_state.ts";

const sdk = await SDK.new("ws://127.0.0.1:9944");
const sharedState = new SharedState(sdk);
const submitDataView = new SubmitDataView(sharedState);

const submitDataViewHtml = SubmitDataView.generateHtml();
const htmlBoilerplate =
	`<script src="https://unpkg.com/htmx.org@2.0.0" integrity="sha384-wS5l5IKJBvK6sPTKa2WZ1js3d947pvWXbPJ1OmWfEuxLgeHcEbjUUA5i9V5ZkpCw" crossorigin="anonymous"></script>`;
const server_app = new Application();
const router = new Router();
router
	.get("/", (context) => {
		context.response.type = "html";
		context.response.body = `${htmlBoilerplate} ${submitDataViewHtml}`;
	})
	.post("/submitDataExecute", async (context) => {
		context.response.type = "html";
		const body = await context.request.body({ type: "form" }).value;
		const data = body.get("Data");
		if (data != null && data != "") {
			const result = await submitDataView.execute(data);
			if (result) {
				context.response.body = result;
			} else {
				context.response.status = Status.NoContent;
			}
		} else {
			context.response.body = "Something went wrong :(";
		}
	})
	.get("/accounts", async (context) => {
		context.response.type = "html";
		context.response.body = `
			<option value="1">Hello World </option>
			<option value="2">Hello World2 </option>
		`;
	});

server_app.use(router.routes());
await server_app.listen({ port: 8000 });
