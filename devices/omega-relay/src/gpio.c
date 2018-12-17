#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <dirent.h>
#include <errno.h>

#include "./gpio.h"

const char* GPIO_PORT_PATH = "/sys/class/gpio";
const char* GPIO_PORT_EXPORT_PATH = "/sys/class/gpio/export";
const char* GPIO_PORT_UNEXPORT_PATH = "/sys/class/gpio/unexport";
const unsigned int GPIO_PORT_MAX_NUMBER = 99;

const char* gpio_port_status_to_string(GPIOPortStatus status) {
	if (status == EXPORTED) {
		return "exported";
	} else {
		return "unexported";
	}
}

const char* gpio_port_direction_to_string(GPIOPortDirection direction) {
	if (direction == IN) {
		return "in";
	} else {
		return "out";
	}
}

const char* get_err_code_msg(GPIOPort *port) {
	char *msg = (char*)malloc(sizeof(char)*100);

	switch (port->err_code) {
		case ERR_CODE_OK:
			msg = "OK";
			break;

		case ERR_CODE_PORT_NUMBER_MAX:
			sprintf(msg, "Port number over maximum: %d", port->number);
			break;

		case ERR_CODE_CLOSE_FAIL:
			sprintf(msg, "Failed to close a device control file "
					"or directory: %s", strerror(port->err_details_num));
			break;

		case ERR_CODE_OPEN_FAIL:
			sprintf(msg, "Failed to open a device control file "
					"or directory: %s", strerror(port->err_details_num));
			break;

		case ERR_CODE_WRITE_FAIL:
			sprintf(msg, "Failed to write to a device control file: %s",
					strerror(port->err_details_num));
			break;

		case ERR_CODE_READ_FAIL:
			sprintf(msg, "Failed to read from a device control file: %s",
					strerror(port->err_details_num));
			break;

		case ERR_CODE_PARSE_DIR_FAIL:
			sprintf(msg, "Failed to parse GPIO port direction value: \"%s\"",
					port->err_details_chars);

			break;

		default:
			sprintf(msg, "Unknown error code: %d", port->err_code);
			break;
	}

	return msg;
}

GPIOPort* new_gpio(const unsigned int number) {
	// Allocate
	GPIOPort *port = (GPIOPort*)malloc(sizeof(GPIOPort));

	// ... Set error code
	port->err_code = ERR_CODE_OK;
	port->err_details_num = 0;
	
	// ... Set number
	port->number = number;

	//     Check number to see if too large
	if (number > GPIO_PORT_MAX_NUMBER) {
		port->err_code = ERR_CODE_PORT_NUMBER_MAX;
		return port;
	}

	// ... Set path
	//     Get length of base path
	//     Add 4 for "gpio"
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

	sprintf(port->control_path, "%s/gpio%d", GPIO_PORT_PATH, number);

	// .. Set direction path
	//     Add 10 for "/direction"
	int dir_path_len = strlen(port->control_path);
	dir_path_len += 10;

	port->direction_control_path = (char*)malloc(sizeof(char)*dir_path_len);
	sprintf(port->direction_control_path, "%s/direction", port->control_path);

	return port;
}


void free_gpio(GPIOPort *port) {
	free((char*)port->control_path);
	free((char*)port->direction_control_path);
	free(port);
}

bool gpio_get_status(GPIOPort *port, GPIOPortStatus *status) {
	// Check if GPIO control directory exists
	DIR *dir = opendir(port->control_path);

	// If exists
	if (dir) {
		*status = EXPORTED;

		// Try to close directory
		if (closedir(dir) < 0) {
			port->err_code = ERR_CODE_CLOSE_FAIL;
			port->err_details_num = errno;

			return false;
		}

		return true;
	} else if (errno == ENOENT) { // If doesn't exist
		*status = UNEXPORTED;

		return true;
	} else {
		port->err_code = ERR_CODE_OPEN_FAIL;
		port->err_details_num = errno;

		return false;
	}
}

bool gpio_set_status(GPIOPort *port, const GPIOPortStatus status) {
	// Determine file path to write
	char *file_path;

	if (status == EXPORTED) {
		file_path = (char*)GPIO_PORT_EXPORT_PATH;
	} else {
		file_path = (char*)GPIO_PORT_UNEXPORT_PATH;
	}
	
	// Open file
	FILE *file = fopen(file_path, "w");
	if (!file) { // If failed to open file
		port->err_code = ERR_CODE_OPEN_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Write to file
	if (fprintf(file, "%d\n", port->number) < 0) { // If failed to write to file
		port->err_code = ERR_CODE_WRITE_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Close file
	if (fclose(file) != 0) { // If failed to close file
		port->err_code = ERR_CODE_CLOSE_FAIL;
		port->err_details_num = errno;

		return false;
	 }

	return true;
}


bool gpio_get_direction(GPIOPort *port, GPIOPortDirection *direction) {	
	// Open file
	FILE *file = fopen(port->direction_control_path, "r");

	if (!file) { // If failed to open file
		port->err_code = ERR_CODE_OPEN_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Read file
	char buffer[4];

	if (fgets(buffer, sizeof(buffer), file) == NULL) { // If failed to read file
		port->err_code = ERR_CODE_READ_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Interpret value
	if (strcmp(buffer, "in\n") == 0) {
		*direction = IN;

		return true;
	} else if (strcmp(buffer, "out") == 0) {
		*direction = OUT;

		return true;
	} else {
		port->err_code = ERR_CODE_PARSE_DIR_FAIL;
		strcpy(port->err_details_chars, buffer);

		return false;
	}

	// Close file
	if (fclose(file) != 0) {
		port->err_code = ERR_CODE_CLOSE_FAIL;
		port->err_details_num = errno;

		return false;
	}

	return true;
}

bool gpio_set_direction(GPIOPort *port, const GPIOPortDirection direction) {
	// Open file
	FILE *file = fopen(port->direction_control_path, "w");

	if (!file) { // If failed to open file
		port->err_code = ERR_CODE_OPEN_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Write file
	if (fprintf(file, "%s\n", gpio_port_direction_to_string(direction)) < 0) {
		// If failed to write
		port->err_code = ERR_CODE_WRITE_FAIL;
		port->err_details_num = errno;

		return false;
	}

	// Close file
	if (fclose(file) != 0) {
		port->err_code = ERR_CODE_CLOSE_FAIL;
		port->err_details_num = errno;

		return false;
	}

	return true;
}
