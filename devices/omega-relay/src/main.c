#include <stdio.h>

#include "./gpio.h"

void main() {
	GPIOPort *port = new_gpio(1);

	if (port->err_code < 0) {
		printf("error creating GPIO port: %s\n",
				get_err_code_msg(port->err_code));
	}

	printf("GPIO port->control_path: %s\n", port->control_path);

	free_gpio(port);
}
