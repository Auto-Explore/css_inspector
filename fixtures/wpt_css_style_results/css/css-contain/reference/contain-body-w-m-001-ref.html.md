# css/css-contain/reference/contain-body-w-m-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/reference/contain-body-w-m-001-ref.html"
}
```

## style[0]

```css

html::before {
    content: "";
    width: 100px;
    height: 100px;
    background: orange;
    display: block;
}
body { margin: 0; }
p {
    margin: 0;
    width: 200px;
    height: 200px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
