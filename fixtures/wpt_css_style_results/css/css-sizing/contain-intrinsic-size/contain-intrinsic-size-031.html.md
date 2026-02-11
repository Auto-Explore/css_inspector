# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-031.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-031.html"
}
```

## style[0]

```css

.test {
  contain: size;
  display: inline-block;
  padding: 0;
  border: 5px solid;
  column-gap: 5px;
}
.columns-60px-1 {
  columns: 60px 1;
}
.columns-120px-1 {
  columns: 120px 1;
}
.columns-60px-2 {
  columns: 60px 2;
}
.cis-none {
  contain-intrinsic-size: none none;
}
.cis-height {
  contain-intrinsic-size: none 50px;
}
.cis-width {
  contain-intrinsic-size: 100px none;
}
.cis-both {
  contain-intrinsic-size: 100px 50px;
}
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
