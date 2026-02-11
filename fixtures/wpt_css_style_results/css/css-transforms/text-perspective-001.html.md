# css/css-transforms/text-perspective-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/text-perspective-001.html"
}
```

## style[0]

```css

* {
  margin: 0;
  padding: 0;
}
.container {
  perspective: 1px;
}
.heading {
  transform: translateZ(-9px) scale(10);
}
h1 {
  font-size: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
