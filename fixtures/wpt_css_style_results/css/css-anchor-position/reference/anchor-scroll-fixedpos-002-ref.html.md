# css/css-anchor-position/reference/anchor-scroll-fixedpos-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-fixedpos-002-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
  height: 2000px;
}

div {
  width: 100px;
  height: 100px;
}

#anchor {
  position: fixed;
  left: 300px;
  top: 300px;
  background: orange;
}

#anchored {
  position: fixed;
  left: 400px;
  top: 300px;
  background: green;
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
