# css/css-gaps/animation/rule-width-interpolation-repeaters.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-width-interpolation-repeaters.html"
}
```

## style[0]

```css

        .parent {
            row-rule-style: solid;
            row-rule-width: 90px;
        }

        .target {
            row-gap: 40px;
            column-gap: 40px;
            row-rule-width: 10px, repeat(2, 20px, 20px), 20px;
            row-rule-style: solid;
            row-rule-color: black;
            flex-wrap: wrap;
            column-rule-width: repeat(2, 20px, 20px);
            column-rule-style: solid;
            column-rule-color: black;
        }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
