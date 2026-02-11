# css/css-anchor-position/anchor-scroll-fixedpos.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-fixedpos.html"
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
  anchor-name: --a1;
  margin: 300px;
  background: orange;
}

#anchored {
  position: fixed;
  position-anchor: --a1;
  left: anchor(right);
  top: anchor(top);
  background: green;
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
