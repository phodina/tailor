//! Implementation {{ peripheral.name }} - {{ peripheral.description }}
use regs::ReadWrite;
use macros;

#[repr(C)]
pub struct {{ peripheral.name|title }} {
    {%if peripheral.registers %}
        {% for register in peripheral.registers %}
    {{ register.name|lower }}: <{% if register.size == 32 %}u32{% elif register.size == 16 %}u16{% else %}u8{% endif %}{% if register.fields|length %}, {{ register.name|upper }}::Register{% endif %}>{% if not forloop.last %},{% endif %}
        {% endfor %}
    {% endif %}
}

register_bitfields! [u32,
    {%if peripheral.registers %}
        {% for register in peripheral.registers %}
                     {{ register.name|upper }} [
                         {% for field in register.fields %}
                         {{ field.name }} OFFSET({{ field.bit_offset }}) NUMBITS({{ field.bit_width }}) []{% if not forloop.last %},{% endif %}
                         {% endfor %}
                     ]{% if not forloop.last %},{% endif %}
        {% endfor %}
    {% endif %}
];

const BASE_ADDRESS: StaticRef<{{ peripheral.name|title }}Registers> =
    unsafe { StaticRef::new({{ peripheral.base_address }} as *const {{ peripheral.name|title }}Registers) };

/// Statically allocated {{ peripheral.name }} driver
pub static mut {{ peripheral.name|upper }}: {{ peripheral.name|title }} = {{ peripheral.name|title }}::new(BASE_ADDRESS);

impl {{ peripheral.name|title }} {

    pub fn new (base_addr: StaticRef<{{ peripheral.name|title }}Registers>) -> mut {{ peripheral.name|title }}{
        // TODO
    }
}
