# css/css-sizing/contain-intrinsic-size/auto-007.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-007.html"
}
```

## style[0]

```css

.test {
  width: max-content;
  height: max-content;
  border: 1px solid;
}
.test::before {
  content: "";
  display: block;
  width: 320px;
  height: 240px;
}
.contain-size {
  contain: size;
}
.auto-width {
  contain-intrinsic-width: auto 1px;
}
.auto-height {
  contain-intrinsic-height: auto 2px;
}
.auto-both {
  contain-intrinsic-size: auto 3px auto 4px;
}
.skip-contents .test {
  content-visibility: hidden;
}
.scroll {
  overflow: scroll;
}
.columns {
  columns: 60px 2;
}
.grid {
  display: grid;
}
.flex {
  display: flex;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
