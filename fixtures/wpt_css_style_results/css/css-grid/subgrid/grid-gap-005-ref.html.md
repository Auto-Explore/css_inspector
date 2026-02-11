# css/css-grid/subgrid/grid-gap-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/grid-gap-005-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:24px/1 Ahem; padding:0; margin:0;
}

span { background:cyan; }
span:nth-child(2n+1) { background:grey; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
