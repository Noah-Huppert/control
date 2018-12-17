#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "./gpio.h"

const char* GPIO_PORT_PATH = "/sys/class/gpio";
const unsigned int GPIO_PORT_MAX_NUMBER = 99;

const char* get_err_code_msg(const unsigned int err_code) {
	switch (err_code) {
		case ERR_CODE_OK:
			return "OK";
			break;

		case ERR_CODE_PORT_NUMBER_MAX:
			return "Port number over maximum";
			break;

		default:
			return "Unknown error number";
			break;
	}
}

GPIOPort* new_gpio(const unsigned int number) {
	// Allocate
	GPIOPort *port = (GPIOPort*)malloc(sizeof(GPIOPort));
	
	// ... Set number
	port->number = number;

	//     Check number to see if too large
	if (number > GPIO_PORT_MAX_NUMBER) {
		port->err_code = ERR_CODE_PORT_NUMBER_MAX;
		return port;
	}

	// ... Set path
	//     Get length of base path
	//     Add 1 for single digit number
	//     Add 1 for null terminating
	//     Add 1 for slash in path
	int path_len = strlen(GPIO_PORT_PATH);
	path_len += 3;
	
	//    Add additional 1 if number is double digits
	if (number > 9) {
		path_len++;
	}

	port->control_path = (char*)malloc(sizeof(char)*path_len);

	strcat(port->control_path, GPIO_PORT_PATH);
	sprintf(port->control_path, "%s/%d", port->control_path, number);

	// Errno
	port->err_code = ERR_CODE_OK;

	return port;
}


void free_gpio(GPIOPort *port) {
	free((char*)port->control_path);
	free(port);
}
