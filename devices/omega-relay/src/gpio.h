#ifndef __GPIO_H__
#define __GPIO_H__

#include <stdbool.h>

/**
 *
 * Interface to the OS system which controls general input output ports
 *
 * # GPIO OS System Overview
 * GPIO ports are controlled via files in the `GPIO_PORT_PATH` directory.
 *
 * Each exported GPIO port gets its own directory named in format:
 *
 * ```
 * gpio<number>
 * ```
 *
 * Where `<number>` is a unsigned integer used to identify the GPIO port.
 *
 * ## Port Initialization
 * Before a GPIO port can be controlled by the OS it must be exported.
 *
 * To export a GPIO port write the port number to the `/export` file.  
 *
 * To unexport a GPIO port write the port number to the `/unexport` file.
 *
 * ## Control Files
 * Inside each GPIO port directory multiple files exist which control
 * the port.
 *
 * ### Direction
 * **File**: `/direction`  
 * **Valid values**: `in` or `out`  
 * 
 * If the port is outputting or receiving signals.  
 *
 * Read this file to determine the direction.  
 * Write to this file to set the direction
 *
 * ### Value
 * **File**: `/value`  
 * **Valid values**: `0` or `1`  
 *
 * If the port is on or off.
 *
 *
 * Read this file to determine the value.  
 * Write to this file to set the value
 */
typedef struct {
	/**
	 * Number to identify GPIO port
	 */
	unsigned int number;

	/**
	 * Path to system control files directory for GPIO port
	 */
	char *control_path;

	/**
	 * Path to direction system control for GPIO port
	 */
	char *direction_control_path;

	/**
	 * Error number
	 */
	int err_code;

	/**
     * Errno value associated with error code
	 */
	int err_details_num;

	/**
	 * Character array value associated with error code
	 */
	char err_details_chars[10];
} GPIOPort;

/**
 * Indicates there is no error.
 */
#define ERR_CODE_OK 0

/**
 * Indicates the provided port number is too large. See GPIO_PORT_MAX_NUMBER 
 * for the maximum (inclusive) allowed port number.
 */
#define ERR_CODE_PORT_NUMBER_MAX -1

/**
 * Indicates an error occurred while closing a device control file 
 * or directory.
 *
 * err_details_num will be set to errno if this error occurs.
 */
#define ERR_CODE_CLOSE_FAIL -2

/**
 * Indicates an error occurred while opening a device control file 
 *
 * err_details_num will be set to errno if this error occurs.* or directory.
 */
#define ERR_CODE_OPEN_FAIL -3

/**
 * Indicates an error occurred while writing to a device control file.
 *
 * err_details_num will be set to errno if this error occurs.
 */
#define ERR_CODE_WRITE_FAIL -4

/**
 * Indicates an error occurred while reading from a device control file.
 *
 * err_details_num will be set to errno if this error occurs.
 */
#define ERR_CODE_READ_FAIL -5

/**
 * Indicates an error occurred while parsing a string value in a
 * GPIOPortDirection enum.
 *
 * err_details_chars will be set to the string value which could not be parsed.
 */
#define ERR_CODE_PARSE_DIR_FAIL -6

/**
 * GPIO system control directory path
 */
extern const char* GPIO_PORT_PATH;

/**
 * GPIO system control file to export ports
 */
extern const char* GPIO_PORT_EXPORT_PATH;

/**
 * GPIO system control file to unexport ports
 */
extern const char* GPIO_PORT_UNEXPORT_PATH;

/**
 * Indicates the maximum (inclusive) allowed port number
 */
extern const unsigned int GPIO_PORT_MAX_NUMBER;

/**
 * Indicates the status of the GPIO port with the OS
 */
typedef enum {
	/**
	 * Indicates port is being controlled by the OS
	 */
	EXPORTED,

	/**
	 * Indicates port is not being controlled by the OS
	 */
	UNEXPORTED,
} GPIOPortStatus;

/**
 * Encodes GPIOPortStatus enum value to string
 * @param status Status to parse
 * @returns "exported" or "unexported"
 */
const char* gpio_port_status_to_string(GPIOPortStatus status);

/**
 * Indicates if GPIO port is sending or receiving signals
 */
typedef enum {
	/**
	 * Indicates port is sending signals
	 */
	OUT,

	/**
	 * Indicates port is receiving signals
	 */
	IN,
} GPIOPortDirection;

/**
 * Encodes GPIOPortDirection enum value to string
 * @param direction Direction to parse
 * @returns "in" or "out"
 */
const char* gpio_port_direction_to_string(GPIOPortDirection direction);

/**
 * Gets message for err_code
 * @param err_code Error number
 * @returns Error message
 */
const char* get_err_code_msg(GPIOPort *port);

/**
 * Initialize a GPIOPort
 * @param port Variable to store GPIO port in
 * @param number GPIO port number
 * @returns Pointer to GPIOPort
 */
GPIOPort* new_gpio(const unsigned int number);

/**
 * Free GPIO port
 */
void free_gpio(GPIOPort *port);

/**
 * Get status of GPIO port
 * @param port GPIO port
 * @param status Variable to hold status in
 * @returns False if failure
 */
bool gpio_get_status(GPIOPort *port, GPIOPortStatus *status);

/**
 * Set status of GPIO port
 * @param port GPIO port
 * @param status Status to set
 * @returns False if failure
 */
bool gpio_set_status(GPIOPort *port, const GPIOPortStatus status);

/**
 * Get direction of GPIO port
 * @param port GPIO port
 * @param direction Variable to store direction in
 * @returns False if failure
 */
bool gpio_get_direction(GPIOPort *port, GPIOPortDirection *direction);

/**
 * Set direction of GPIO port
 * @param port GPIO port
 * @param direction Direction to set
 * @returns False if failure
 */
bool gpio_set_direction(GPIOPort *port, const GPIOPortDirection direction);

/**
 * Get value of GPIO port
 * @param port GPIO port
 * @param value Variable to store value in
 * @returns False if failure
 */
bool gpio_get_value(GPIOPort *port, bool *value);

/**
 * Set value of GPIO port
 * @param port GPIO port
 * @param value Value to set, True for high state, false for low state
 * @returns False if failure
 *
 */
bool gpio_set_value(GPIOPort *port, const bool value);

#endif
