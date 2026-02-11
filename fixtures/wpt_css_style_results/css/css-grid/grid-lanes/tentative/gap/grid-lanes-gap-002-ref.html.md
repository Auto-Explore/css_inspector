# css/css-grid/grid-lanes/tentative/gap/grid-lanes-gap-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/gap/grid-lanes-gap-002-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:25px/1 "Courier New", monospace; padding:0; margin:0;
}

grid {
  display: inline-grid;
  gap: 0;
  grid-template-columns: auto min-content repeat(2,auto);
  color: #444;
  border: 1px solid;
  padding: 2px;
}

item {
  background-color: #444;
  color: #fff;
  padding: 20px;
  margin: 3px;
  border: 5px solid blue;
  place-self: start;
  grid-row: span 2;
  display: block;
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
