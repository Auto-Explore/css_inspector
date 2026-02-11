# css/css-ui/outline-022.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-022.html"
}
```

## style[0]

```css

body {
  --outline-width: 10px;
  --square-width: 80px;
}
#container {
  width: var(--square-width);
  padding: var(--outline-width);
}
#target {
  font-family: Ahem;
  font-size: var(--square-width);
  outline: solid var(--outline-width) green;
  color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
