#!/bin/bash

cd /tmp
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
