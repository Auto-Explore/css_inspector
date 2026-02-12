# css/css-gaps/animation/rule-color-interpolation-repeaters-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-color-interpolation-repeaters-001.html"
}
```

## style[0]

```css

        .parent {
            row-rule-style: solid;
            row-rule-color: white;
        }

        .target {
            row-gap: 40px;
            column-gap: 40px;
            row-rule-width: 10px;
            row-rule-style: solid;
            row-rule-color: black, repeat(2, red, blue), black;
            flex-wrap: wrap;
            column-rule-width: 10px;
            column-rule-style: solid;
            column-rule-color: repeat(2, black, red);
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
