#ifndef {{ file_define }}
#define {{ file_define }}

#include <stdint.h>
#define REG_BIT_DEFN(start, end) ((start<<32)|(end-start+1))

{% if peripheral.instances|length > 1 %}
enum {{ peripheral.name }}_Base {
{% for instance in peripheral.instances %}
    {{ instance.name }} = {{ instance.baseAddr }}, 
{% endfor %}
};
{% endif %}

{% for register in peripheral.registers %}
{% if register.fields|length > 1 %}
//! {{ register.name }} - {{ register.description }}
class {{ register.name }} {
    enum{
{% for field in register.fields %}
        {{ field.name }} = REG_BIT_DEFN({{ field.bitOffset }},{{ field.bitWidth }}), //! {{ field.description }}
{% endfor %}
    };

public:
    
{% for field in register.fields %}
    void set{{ field.name }} () { reg |= {{ field.name }}; }
    bool get{{ field.name }} () { return reg & {{ field.name }}; } 
    
{% endfor %}

private:
    {% if register.size == "0x20" %}
    volatile {% if register.access == "read" %}const {% endif %}uint32_t reg;
    {% elif register.size == "0x10" %}
    volatile {% if register.access == "read" %}const {% endif %}uint16_t reg;
    {% elif register.size == "0x8" %}
    volatile {% if register.access == "read" %}const {% endif %}uint8_t reg;
    {% endif%}
};

{% endif %}
{% endfor %}

//! {{ peripheral.name }} - {{ peripheral.description }}
class {{ ip_name }} {

public:
    {{ ip_name }}();
    
{% for register in peripheral.registers %}
    {% if register.fields|length > 1 %}
    {{ register.name }} get{{ register.name }}() { return {{ register.name|lower }}; }
    {% endif %}
{% endfor %}
private:
{% for register in peripheral.registers %}
    {% if register.fields|length > 1 %}
    {{ register.name }} {{ register.name|lower }};
    {% else%}
        {% if register.size == "0x20" %}
    volatile uint32_t {{ register.name|lower }};
        {% elif register.size == "0x10" %}
    volatile uint16_t {{ register.name|lower }};
        {% elif register.size == "0x8" %}
    volatile uint8_t {{ register.name|lower }};
        {% endif%}
    {% endif %}
{% endfor %}
};

#endif
 
