# css/CSS2/normal-flow/negative-margin-001.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/negative-margin-001.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
}
html {
  margin-left: 10px;
}
outer {
  display: block;
  border: blue 10px solid;
  width: 100px;
}
inner {
  display: block;
  border: orange 10px solid;
  margin-left: -20px;
  margin-right: -50px;
  height: 10px;
}
inner.bfc {
  overflow: hidden;
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
