import { getAddress, parse } from '../yeelight';

test('yeelight discover payload is parsed properly', () => {
    // tslint:disable: no-trailing-whitespace
    const stringPayload = `HTTP/1.1 200 OK
    Cache-Control: max-age=3600
    Date: 
    Ext:
    Location: yeelight://192.168.1.46:55443
    Server: POSIX UPnP/1.0 YGLC/1
    id: 0x0000000007fb4b9b
    model: color
    fw_ver: 35
    support: get_prop set_default set_power toggle set_bright start_cf stop_cf set_scene cron_add cron_get cron_del set_ct_abx set_rgb set_hsv set_adjust adjust_bright adjust_ct adjust_color set_music set_name
    power: off
    bright: 90
    color_mode: 2
    ct: 4000
    rgb: 16729088
    hue: 16
    sat: 100
    name: Chambre`;

    const parsed = parse(stringPayload);

    expect(parsed.date).toBeUndefined();
    expect(parsed.ext).toBeUndefined();
    expect(parsed.location).toBe('yeelight://192.168.1.46:55443');
    expect(getAddress(parsed).host).toBe('192.168.1.46');
    expect(getAddress(parsed).port).toBe(55443);
    expect(parsed.power).toBe(false)
    expect(parsed.rgb).toBe(16729088);
})

