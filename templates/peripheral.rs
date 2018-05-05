// {{ peripheral.name }} - {{ peripheral.description }}

#[repr(C,packed)]
pub struct {{ peripheral.name }} {

    {%if peripheral.registers %}
        {% for register in peripheral.registers %}
            ARRAY HURRAY
            {% if register.is_left %}
            TRUE
            {% endif%}
        {% endfor %}
    {% endif %}
}

impl {{ peripheral.name }} {
    

}
