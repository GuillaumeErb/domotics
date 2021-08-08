import express from 'express';
import path from 'path';
import { YeelightMonitor } from './yeelight';

const PORT = process.env.PORT || 8000;

const app = express();

const yeelightMonitor = new YeelightMonitor();
{
    const abortController = new AbortController();
    yeelightMonitor.startDiscovery(abortController.signal);
    setTimeout(() => abortController.abort(), 1000 * 2);
}

app.get("/api/lights", (req, res) => {
    console.log(JSON.stringify(req.query))
    if (req.query.refresh === "true") {
        const abortController = new AbortController();
        yeelightMonitor.startDiscovery(abortController.signal);
        setTimeout(() => {
            abortController.abort();
            res.json(yeelightMonitor.getDevices());
        }, 1000 * 2);
    } else {
        res.json(yeelightMonitor.getDevices());
    }
});

app.get("/api/lights/:id", (req, res) => {
    const { id } = req.params;
    const device = yeelightMonitor.getDevice(parseInt(id, 10));
    if (device) {
        res.json(device)
    } else {
        res.status(400);
    }
});

app.get("/api/lights/:id/toggle", async (req, res) => {
    const { id } = req.params;
    await yeelightMonitor.toggle(parseInt(id, 10));
    res.sendStatus(200);
});

app.get('/', (req, res) => {
    console.log('get /')
    res.sendFile(path.resolve(__dirname, '../../domotics-frontend/build', 'index.html'));
});

app.use('/www', express.static(path.join(__dirname, '../../domotics-frontend/build')));

app.listen(PORT, () => {
    // tslint:disable-next-line:no-console
    console.log(`Server listening on ${PORT}`);
});