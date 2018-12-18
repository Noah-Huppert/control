#include <stdio.h>
#include <curl/curl.h>

#include "./gpio.h"

void main() {
	// Create
	GPIOPort *port = new_gpio(1);

	if (port->err_code < 0) {
		printf("error creating GPIO port: %s\n",
				get_err_code_msg(port));
		return;
	}

	// Get status
	GPIOPortStatus status;
	if (!gpio_get_status(port, &status)) {
		printf("error getting GPIO port status: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio port status: %s\n", gpio_port_status_to_string(status));

	// Export
	if (!gpio_set_status(port, EXPORTED)) {
		printf("error exporting GPIO port: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("exported\n");

	// Get status
	if (!gpio_get_status(port, &status)) {
		printf("error getting GPIO port status: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio port status: %s\n", gpio_port_status_to_string(status));

	// Get direction
	GPIOPortDirection direction;
	if (!gpio_get_direction(port, &direction)) {
		printf("error getting GPIO port direction: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio port direction: %s\n", gpio_port_direction_to_string(direction));

	// Set direction
	if(!gpio_set_direction(port, IN)) {
		printf("error setting GPIO port direction: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("in-ed\n");

	// Get direction
	if (!gpio_get_direction(port, &direction)) {
		printf("error getting GPIO port direction: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio port direction: %s\n", gpio_port_direction_to_string(direction));

	// Set direction
	if(!gpio_set_direction(port, OUT)) {
		printf("error setting GPIO port direction: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("out-ed\n");

	// Get direction
	if (!gpio_get_direction(port, &direction)) {
		printf("error getting GPIO port direction: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio port direction: %s\n", gpio_port_direction_to_string(direction));

	// Set value
	if (!gpio_set_value(port, true)) {
		printf("error setting GPIO port value: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("set on\n");

	// Get value
	bool value;
	if (!gpio_get_value(port, &value)) {
		printf("error getting GPIO port value: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio value: %d\n", value);

	// Set value
	if (!gpio_set_value(port, false)) {
		printf("error setting GPIO port value: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("set off\n");

	// Get value
	if (!gpio_get_value(port, &value)) {
		printf("error getting GPIO port value: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("gpio value: %d\n", value);

	// Unexport
	if (!gpio_set_status(port, UNEXPORTED)) {
		printf("error unexporting GPIO port: %s\n",
				get_err_code_msg(port));
		return;
	}

	printf("unexported\n");

	// Free
	free_gpio(port);
}
