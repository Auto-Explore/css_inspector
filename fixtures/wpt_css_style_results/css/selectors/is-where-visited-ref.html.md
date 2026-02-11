# css/selectors/is-where-visited-ref.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-visited-ref.html"
}
```

## style[0]

```css

  :visited, :link {
    color: black;
  }

  #parent1 .a, #parent1 .b, #parent1 .c {
    color: green;
  }
  #parent1 .d, #parent1 .f {
    color: magenta;
  }

  #parent2 .b, #parent2 .e {
    color: tomato;
  }

  #parent3 .c, #parent3 .f {
    color: skyblue;
  }

  #parent4 .c, #parent4 .f {
    color: blue;
  }

  #parent3 .b, #parent3 .e, #parent4 .b, #parent4 .e {
    color: maroon;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
