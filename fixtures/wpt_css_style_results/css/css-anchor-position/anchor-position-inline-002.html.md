# css/css-anchor-position/anchor-position-inline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-inline-002.html"
}
```

## style[0]

```css

#container {
  position: relative;
  font-family: Ahem;
  font-size: 10px;
  line-height: 1;
  width: 10em;
}
#anchor1 {
  anchor-name: --a1;
}
.target {
  position: absolute;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
