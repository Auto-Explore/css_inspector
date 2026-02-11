# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-justify-content-003.html"
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
  padding: 2px;
  width: 100px;
}

item {
  background-color: #444;
  color: #fff;
}

.tall { padding-left: 30px; }

.safe {
  width: 10px;
  margin-right: 50px;
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
