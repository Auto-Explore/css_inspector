# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-002-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
}

grid {
  display: inline-grid;
  gap: 1px 2px;
  grid-template-columns: repeat(4,20px);
  grid-template-rows: 1em auto;
  color: #444;
  border: 1px solid;
  padding: 2px;
  height: 100px;
  writing-mode: vertical-lr;
  align-items: start;
}

item {
  background-color: #444;
  color: #fff;
}

.tall { padding-right:30px; grid-row:span 2; }

.safe {
  height: 10px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
