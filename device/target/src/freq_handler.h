/**
 * @file freq_handler.h
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2025-03-05
 * @brief Handle frequency packet generation and processing
 */


#ifndef PACKET_HANDLER_H
#define PACKET_HANDLER_H

#include <stdint.h>
#include <stdbool.h>

#include "custom_can_protocol/packet_processing.h"

#include "freq_info.h"

/**
 * @brief Initialise the frequency handler
 *
 * @return 0 if successful
 */
int freq_handler_init(void);

/**
 * @brief Update the frequency value
 *
 * @return true if frequency has changed
 */
bool freq_handler_update(void);

/** 
 * @brief Convert a device_select value to a frequency type
 * @param value the device_select value
 * 
 * @return the frequency type value
 */
freqType_t freq_handler_convert_to_type(uint8_t value);

/**
 * @brief Assemble a frequency handler payload to be sent
 * @param buffer the buffer to load into
 *
 * @return the length of the buffer
 */
uint16_t freq_handler_packet_assemble(uint8_t* buffer);

/**
 * @brief A callback to handle incoming frequency set packets
 * @param payload the packet payload buffer
 * @param payloadLen the packet payload length
 *
 * @return the result of the processing
 */
packetProcessingResult_t freq_handler_packet_cb(uint8_t* payload, uint16_t payloadLen);

#endif // PACKET_HANDLER_H