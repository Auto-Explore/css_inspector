# css/css-color/system-color-compute.html

```json
{
  "format_version": 3,
  "file": "css/css-color/system-color-compute.html"
}
```

## style[0]

```css

.parent {
    border: 1px solid black;
    width: 100px;
    height: 170px;
    margin: 5px;
    padding: 5px;
    color-scheme: light;
}

.child {
    position: relative;
    border: 1px solid black;
    width: 80px;
    height: 50px;
    margin: 4px;
    padding: 4px;
    color-scheme: dark;
}

.specified {
    color: Menu;
    background-color: Menu;
    box-shadow: 2px 2px Menu;
    text-shadow: 2px 2px Menu;
    border-color: Menu;
    column-rule-color: Menu;
    outline-color: Menu;
    caret-color: Menu;
    fill: Menu;
    stroke: Menu;
}

.inherit {
    color: inherit;
    background-color: inherit;
    box-shadow: inherit;
    text-shadow: inherit;
    border-color: inherit;
    column-rule-color: inherit;
    outline-color: inherit;
    caret-color: inherit;
    fill: inherit;
    stroke: inherit;
}
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
