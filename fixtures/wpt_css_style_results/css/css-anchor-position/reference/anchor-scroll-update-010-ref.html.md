# css/css-anchor-position/reference/anchor-scroll-update-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-update-010-ref.html"
}
```

## style[0]

```css

.anchor {
  width: 20px;
  height: 20px;
  background: magenta;
}

.positioned {
  background: purple;
  width: 20px;
  height: 20px;
}

.flex {
  display: flex;
}

.abs-cb {
  position: relative;
  width: 200px;
  height: 200px;
  border: 1px solid;
}

.scroll {
  overflow: scroll;
}

.outer {
  width: 200px;
  height: 200px;
}

.inner {
  width: 150px;
  height: 150px;
}

.filler {
  width: 1px;
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
