# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-004.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
}

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  gap: 1px 2px;
  grid-template-rows: repeat(4,20px);
  background: content-box silver;
  border: 1px solid;
  padding: 0 3px 2px 0;
  width: 100px;
  height: 120px;
  align-content: center;
  writing-mode: vertical-rl;
}

item {
  background-color: #444;
  color: #fff;
}

.tall { padding: 11px 3px 13px 7px; }

.safe {
  height: 10px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
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
