export interface Light {
    id: number;
    address: string;
    power: boolean;
    bright: number;
    rgb: number;

}

export const getAllLightsAsync = async (): Promise<Light[]> => {
    const rawLights = await fetch("http://localhost:8000/lights");
    return rawLights.json();
}

export const toggleLight = async (id: number): Promise<void> => {
    await fetch(`http://localhost:8000/lights/${id}/toggle`);
}