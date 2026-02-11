# css/css-anchor-position/anchor-query-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-query-fallback.html"
}
```

## style[0]

```css

#container {
  position: relative;
  display: flex;
  flex-wrap: wrap;
  width: 300px;
}

.flex-item {
  width: 100px;
  height: 50px;
  flex: auto;
}

#a1 {
  anchor-name: --a1;
  background: orange;
}
#a2 {
  anchor-name: --a2;
  background: purple;
}

.target {
  position: absolute;
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
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
