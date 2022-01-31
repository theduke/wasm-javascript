import * as host_api from 'host_api';

globalThis.hostcallstr = host_api.hostcallstr;

export function jseval(code) {
	return eval(code).toString()
}
