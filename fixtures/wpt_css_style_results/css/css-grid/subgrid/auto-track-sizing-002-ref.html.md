# css/css-grid/subgrid/auto-track-sizing-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/auto-track-sizing-002-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}

.grid {
  display: inline-grid;
  max-width: 260px;
  padding: 1px 5px;
  border: 1px solid;
  grid-template-columns: 100px;
}
.subgrid {
  display: grid;
  grid-template-columns: 100%;
  margin: 0px 10px 0px 10px;
  background: grey;
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
