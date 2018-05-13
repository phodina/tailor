{% if peripheral.registers is iterable %}
    TRUE
    {% for register in peripheral.registers %}
        {% if register is defined %}
            {{ register }}
        {% endif %}
    {% endfor %}
{% endif %}
