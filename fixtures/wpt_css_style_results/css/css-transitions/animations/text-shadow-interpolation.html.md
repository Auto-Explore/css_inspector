# css/css-transitions/animations/text-shadow-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/animations/text-shadow-interpolation.html"
}
```

## style[0]

```css

.parent {
  text-shadow: 30px 10px 30px orange;
}

.target {
  display: inline-block;
  font-size: 60pt;
  margin-right: 20px;
  margin-bottom: 30px;
  color: green;
  text-shadow: 10px 30px 10px orange;
}

.expected {
  margin-right: 40px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
