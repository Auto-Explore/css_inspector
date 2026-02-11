# css/css-values/calc-background-image-gradient-1.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-background-image-gradient-1.html"
}
```

## style[0]

```css


p {
    height: 50px; width: 200px;
    border: thin solid;
}

#one { background-image: radial-gradient(circle farthest-side at calc(50px + 50%) calc(100% - 30px), red, green); }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
