# css/css-borders/border-radius-currentcolor-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/border-radius-currentcolor-ref.html"
}
```

## style[0]

```css

:root {
  --color: #fff;
  background: var(--color);
  color: var(--color);
}
div {
  display: block;
  width: 51px;
  height: 51px;
  border-radius: 50%;
  background-color: #323232;
  border: 10px solid var(--color);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
