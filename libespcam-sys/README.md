Bindgen call
============

Used esp32-camera release 2.0.0
works...

bindgen.exe --no-rustfmt-bindings .\esp32-camera\driver\include\esp_camera.h -o .\src\bindings.rs -- -I.\.embuild\platformio\packages\framework-espidf\components\esp_common\include\ -I.\.embuild\platformio\packages\toolchain-xtensa-esp32\xtensa-esp32-elf\include\ -I.\.embuild\platformio\packages\framework-espidf\components\driver\include\ -I.\.embuild\platformio\packages\framework-espidf\components\esp_system\include\ -I.\.embuild\platformio\packages\framework-espidf\components\soc\esp32\include\ -I.\.embuild\platformio\packages\framework-espidf\components\hal\include\ -I.\.embuild\platformio\packages\framework-espidf\components\xtensa\include\ -I. -I.\.embuild\platformio\packages\framework-espidf\components\soc\include\ -DCONFIG_IDF_TARGET_ESP32 -I.\.embuild\platformio\packages\framework-espidf\components\esp32\include\ -I.\.embuild\platformio\packages\framework-espidf\components\esp_rom\include\ -I.\esp32-camera\conversions\include\ -I.\esp32-camera\driver\include\

