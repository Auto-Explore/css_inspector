# css/css-contain/contain-body-dir-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-body-dir-001.html"
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
p {
    margin: 0;
    direction: ltr;
}
body {
    margin: 0 auto 0 0;
    width: 200px;
    height: 200px;
    direction: rtl;
    contain: layout;
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
