# css/css-writing-modes/text-combine-upright-value-digits2-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-combine-upright-value-digits2-001.html"
}
```

## style[0]

```css

@font-face {
  font-family: tcu-font;
  src: url("/fonts/tcu-font.woff");
}

.test {
  writing-mode: vertical-rl;
  font-size: 5em;
  font-family: tcu-font;
}

.tcy {
  text-combine-upright: digits 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-combine-upright”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
