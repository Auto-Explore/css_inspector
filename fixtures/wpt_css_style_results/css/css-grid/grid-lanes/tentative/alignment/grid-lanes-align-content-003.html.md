# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-003.html"
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
  grid-template-rows: repeat(4,auto);
  background: content-box silver;
  border: 1px solid;
  padding: 0 3px 2px 0;
  width: 100px;
  height: 120px;
  align-content: center;
  justify-items: start;
}

item {
  background-color: #444;
  color: #fff;
}

.tall { padding: 3px 11px 1px 13px; }

.safe {
  height: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
