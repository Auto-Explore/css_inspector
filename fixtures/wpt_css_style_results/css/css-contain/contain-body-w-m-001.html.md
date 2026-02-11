# css/css-contain/contain-body-w-m-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-body-w-m-001.html"
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
p { writing-mode: horizontal-tb; margin: 0;}
body {
    margin: 0;
    width: 200px;
    height: 200px;
    writing-mode: vertical-rl;
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
