import { BACKEND_BASE_URL } from "../config";

export interface Light {
    id: number;
    address: string;
    power: boolean;
    bright: number;
    rgb: number;

}

export interface GetAllLightsOptions {
    refresh?: boolean;
}

export const getAllLightsAsync = async (options?: GetAllLightsOptions): Promise<Light[]> => {
    var url = `${BACKEND_BASE_URL}/lights`;
    if (options?.refresh) {
        url += "?refresh=true"
    }
    const rawLights = await fetch(url);
    return rawLights.json();
}

export const toggleLight = async (id: number): Promise<void> => {
    await fetch(`${BACKEND_BASE_URL}/lights/${id}/toggle`);
}