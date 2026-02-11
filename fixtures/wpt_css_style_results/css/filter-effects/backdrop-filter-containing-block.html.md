# css/filter-effects/backdrop-filter-containing-block.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-containing-block.html"
}
```

## style[0]

```css

.container {
  width: 200px;
  backdrop-filter: invert(1);
}

.fixed-child {
  position: fixed;
  top: 0;
  background-color: green;
  width: 100%;
  height: 200px;
}

.absolute-child {
  position: absolute;
  top: 0;
  left: 210px;
  background-color: red;
  width: 100%;
  height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
