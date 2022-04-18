Bindgen call
============

Used esp32-camera relese 2.0.0
Still some includes and definitions missing...

bindgen.exe .\esp32-camera\driver\include\esp_camera.h -o .\src\bindings.rs -- -I.\.embuild\platformio\packages\framework-espidf\components\esp_common\inc
lude\ -I.\.embuild\platformio\packages\toolchain-xtensa-esp32\xtensa-esp32-elf\include\ -I.\.embuild\platformio\packages\framework-espidf\components\driver\include\ -I.\.embuild\platformio\packages\framework-espidf\compon
ents\esp_system\include\ -I.\.embuild\platformio\packages\framework-espidf\components\soc\esp32\include\ -I.\.embuild\platformio\packages\framework-espidf\components\hal\include\ -I.\.embuild\platformio\packages\framework
-espidf\components\xtensa\include\ -I. -I.\.embuild\platformio\packages\framework-espidf\components\soc\include\ -DCONFIG_IDF_TARGET_ESP32 -I.\.embuild\platformio\packages\framework-espidf\components\esp32\include\

