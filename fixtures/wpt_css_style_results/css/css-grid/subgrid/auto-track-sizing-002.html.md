# css/css-grid/subgrid/auto-track-sizing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/auto-track-sizing-002.html"
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
  grid-template-rows: subgrid;
  grid-template-columns: 100%;
  margin: 0px 5px 0px 10px;
}

.inner {
  margin: 0px 5px 0px 0px;
  background: grey;
}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
