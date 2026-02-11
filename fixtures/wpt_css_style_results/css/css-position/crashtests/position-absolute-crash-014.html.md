# css/css-position/crashtests/position-absolute-crash-014.html

```json
{
  "format_version": 3,
  "file": "css/css-position/crashtests/position-absolute-crash-014.html"
}
```

## style[0]

```css

#htmlvar00005 {
  filter: drop-shadow(-1px -1px 1px yellow);
  float: left;
  line-height: 71vw;
}

#htmlvar00006 {
  overflow-x: scroll;
  position: absolute;
}

.class8 {
  font: 52px sans-serif;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
