import * as webxr from "webxr";

const app = new webxr.App();
app.initialize()
    .then(res => {
        console.log(res);
        app.run();
    });
