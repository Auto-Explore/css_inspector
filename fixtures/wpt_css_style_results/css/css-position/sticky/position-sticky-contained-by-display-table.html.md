# css/css-position/sticky/position-sticky-contained-by-display-table.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-contained-by-display-table.html"
}
```

## style[0]

```css

.group {
  display: inline-block;
}

.container {
  height: 1000px;
  width: 50px;
  margin-right: 10px;
}

#scroll-container {
  height: 300px;
  width: 500px;
  overflow: hidden;
  position: relative;
}

.sticky {
  position: sticky;
  height: 50px;
  width: 50px;
  background: green;
  top: 0;
}

#scroll-indicator {
    display: inline-block;
    position: absolute;
    top: 0px;
    width: 300px;
    height: 50px;
    background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
