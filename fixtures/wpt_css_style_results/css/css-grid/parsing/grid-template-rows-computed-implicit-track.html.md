# css/css-grid/parsing/grid-template-rows-computed-implicit-track.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/parsing/grid-template-rows-computed-implicit-track.html"
}
```

## style[0]

```css

#target {
  display: grid;
  height: 1px;
  font-size: 1px;
}
#item {
  grid-row: auto / 1;
  height: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
