# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-001.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
}

grid {
  display: inline-grid-lanes;
  gap: 1px 2px;
  grid-template-columns: repeat(4,20px);
  color: #444;
  border: 1px solid;
  padding: 2px;
  height: 100px;
}

item {
  background-color: #444;
  color: #fff;
}

.safe {
  height: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
