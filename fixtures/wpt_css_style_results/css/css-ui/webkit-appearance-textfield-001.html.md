# css/css-ui/webkit-appearance-textfield-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/webkit-appearance-textfield-001.html"
}
```

## style[0]

```css

 #container { width: 500px; }
 /*
  * If the value being tested is not supported, then 'none' is used instead,
  * which is intended to fail the test.
  */
 #container > * { -webkit-appearance: none; -webkit-appearance: textfield; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
