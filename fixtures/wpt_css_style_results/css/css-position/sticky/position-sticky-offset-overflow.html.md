# css/css-position/sticky/position-sticky-offset-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-offset-overflow.html"
}
```

## style[0]

```css

.container {
  overflow-y: scroll;
  width: 100px;
  height: 100px;
}

.box {
  background-color: green;
  height: 50px;
  width: 50px;
}

.sticky {
  position: sticky;
  top: 200px; /* Forces the sticky position element below the overflow. */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
