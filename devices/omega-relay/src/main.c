#include <stdio.h>

#include "./gpio.h"

void main() {
	GPIOPort *port = new_gpio(1);

	if (port->err_code < 0) {
		printf("error creating GPIO port: %s\n",
				get_err_code_msg(port));
		return;
	}

	GPIOPortStatus status;
	if (!gpio_get_status(port, &status)) {
		printf("error getting GPIO port status: %s\n",
				get_err_code_msg(port));
		return;
	}

	if (status == EXPORTED) {
		printf("gpio port status: EXPORTED\n");
	} else {
		printf("gpio port status: UNEXPORTED\n");
	}

	if (!gpio_set_status(port, EXPORTED)) {
		printf("error exporting GPIO port: %s\n",
				get_err_code_msg(port));
		return;
	}

	free_gpio(port);
}
