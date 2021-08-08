import dgram from 'dgram';
import { createConnection } from 'net';

export class YeelightMonitor {
    private devices = new Map<number, YeelightStatus>();

    public startDiscovery = (abortSignal: AbortSignal) => {
        const diagram = "M-SEARCH * HTTP/1.1\r\n MAN: \"ssdp:discover\"\r\n wifi_bulb";
        const socket = dgram.createSocket({ type: 'udp4', signal: abortSignal });

        socket.bind(1982, '0.0.0.0', () => {
            socket.setBroadcast(true);
            socket.addMembership("239.255.255.250");
        });
        socket.send(diagram, 1982, '239.255.255.250', (e) => {
            if (e) { console.log(`error raised: ${e}`); }
        });
        socket.on('message', (msg, rinfo) => {
            const messageString = msg.toString();
            console.log(`Received message: \n${messageString.split('\n')[0]}`)
            if (msg.includes('M-SEARCH')) {
                return;
            }
            try {
                const yeelightStatus = parse(messageString);
                console.log(`Adding ${yeelightStatus.id}`)
                this.devices.set(yeelightStatus.id, yeelightStatus);
            } catch (e) {
                console.warn(`Could not parse ${msg}:\n ${e}`);
            }
        });
    }

    public getDevices = (): YeelightStatus[] => {
        return Array.from(this.devices.values());
    }

    public getDevice = (id: number): YeelightStatus | undefined => {
        return this.devices.get(id);
    }

    public toggle = async (id: number) => {
        return new Promise<void>((resolve, reject) => {
            const device = this.getDevice(id);
            if (device === undefined) {
                return;
            }
            const address = getAddress(device);
            const client = createConnection(address.port, address.host, () => {
                client.end(JSON.stringify(
                    {
                        id: 1,
                        method: "toggle",
                        params: [""]
                    }
                ) + '\r\n', () => {
                    resolve();
                });
            });
        });
    }
}

export interface YeelightStatus {
    cacheControl: string;
    location: string;
    date?: string;
    ext?: string;
    server?: string;
    id: number;
    model: string;
    firmwareVersion: string;
    support: string;
    power: boolean;
    bright: number;
    colorMode: string;
    colorTemperature: string;
    rgb: number;
    hue: string;
    saturation: string;
    name: string;
}

export const getAddress = (status: YeelightStatus): { host: string, port: number } => {
    const split = status.location.slice(11).split(':');
    const host = split[0];
    const port = parseInt(split[1], 10);
    return { host, port };
};

export const parse = (payloadString: string): YeelightStatus => {
    const payloadMap = new Map(payloadString.toLowerCase()
        .trim()
        .split('\n')
        .slice(1)
        .map(line => line.split(':')
            .map(item => item.trim())
        )
        .map(lineSplit =>
            [
                lineSplit[0],
                lineSplit.length === 1 || !lineSplit[1]
                    ? undefined
                    : lineSplit.slice(1).join(':')
            ]
        )
    );

    return {
        cacheControl: getOrThrow(payloadMap, 'cache-control'),
        location: getOrThrow(payloadMap, 'location'),
        date: payloadMap.get('date'),
        ext: payloadMap.get('ext'),
        server: payloadMap.get('server'),
        id: parseInt(getOrThrow(payloadMap, 'id'), 16),
        model: getOrThrow(payloadMap, 'model'),
        firmwareVersion: getOrThrow(payloadMap, 'fw_ver'),
        support: getOrThrow(payloadMap, 'support'),
        power: getOrThrow(payloadMap, 'power') === "on",
        bright: parseInt(getOrThrow(payloadMap, 'bright'), 10),
        colorMode: getOrThrow(payloadMap, 'color_mode'),
        colorTemperature: getOrThrow(payloadMap, 'ct'),
        rgb: parseInt(getOrThrow(payloadMap, 'rgb'), 10),
        hue: getOrThrow(payloadMap, 'hue'),
        saturation: getOrThrow(payloadMap, 'sat'),
        name: getOrThrow(payloadMap, 'name'),
    }
}

const getOrThrow = (map: Map<string, string | undefined>, key: string): string => {
    const value = map.get(key)
    if (value !== undefined) {
        return value;
    } else {
        throw Error(`Could not find ${key} in payload`);
    }
}
