# css/css-variables/variable-declaration-42.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-42.html"
}
```

## style[0]

```css

p {
  color: red;
}
p {
  color: orange;
  --a-长-name-that-might-be-longer-than-you\27 d-normally-use: green;
  color: var(--a-长-name-that-might-be-longer-than-you\27 d-normally-use);
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
      "message": "Invalid property name in declaration.",
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
