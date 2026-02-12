# css/css-gaps/animation/row-rule-color-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/row-rule-color-interpolation.html"
}
```

## style[0]

```css

.parent {
  row-rule-color: blue;
}

.target {
  display: flex;
  width: 20px;
  row-gap: 10px
  height: 100px;
  background-color: black;
  color: orange;
  row-rule-style: solid;
  row-rule-width: 10px;
  row-rule-color: yellow;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
