
/**
 * @file main.c
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2024-12-23
 * @brief Main file for the flight simulator radio panel project
 */


#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>


#include "avr_extends/GPIO.h"
#include "avr_extends/delay.h"
#include "avr_extends/UART.h"

#include <avr/interrupt.h>

#include "custom_can_protocol/packet_handler.h"
#include "custom_can_protocol/packet_processing.h"

#include "freq_handler.h"
#include "device_select.h"

bool debug = true;

pin_t pin13;

int getc_new(void) {
    if (UART_data_available()) {
        return getchar();
    } else {
        return EOF;
    }
}

void setup(void) {
    pin13 = PIN(PORTB, 5);
    GPIO_pin_init(pin13, OUTPUT);

    UART_init_stdio(115200);
    printf("Radio: 1\n");
    delay_ms(3000);

    freq_handler_init();
    device_select_init();

    struct PacketProcessor freqPacketProcessor = {
        .identifier = 0x01,
        .packet_processing_cb = freq_handler_packet_cb
    };

    struct PacketProcessor channelPacketProcessor = {
        .identifier = 0x04,
        .packet_processing_cb = device_select_packet_cb
    };


    packet_processing_add_callback(freqPacketProcessor);
    packet_processing_add_callback(channelPacketProcessor);
}

int main(void) {
    setup();
    sei();

    while (true) {
        if (freq_handler_update()) {
            uint8_t payloadBuf[10] = { 0 };
            uint16_t payloadSize = freq_handler_packet_assemble(payloadBuf);

            packet_send(putchar, payloadBuf, payloadSize, 0x01);
        }

        if (device_select_update()) {
            uint8_t payloadBuf[5] = { 0 };
            uint16_t payloadLen = device_select_packet_assemble(payloadBuf);

            packet_send(putchar, payloadBuf, payloadLen, 0x04);
        }

        uint8_t readBuf[50];
        uint16_t length = packet_receive(getc_new, readBuf);


        if (length > 0) {
            packetProcessingResult_t result = packet_processing_process(readBuf, length);

            if (result != PROCESS_COMPLETE) {
                printf("Packet processing error: %d\n", result);
            }
        }

        delay_ms(50);
    }
}
