# css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/grid-lanes-align-content-004-ref.html"
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
  grid-template-columns: 1ch auto;
  grid-template-rows: repeat(4,20px);
  background: content-box silver;
  border: 1px solid;
  padding: 2px;
  width: 100px;
  justify-items: start;
  writing-mode: vertical-rl;
}

item {
  background-color: #444;
  color: #fff;
}

.tall { grid-column: span 2; padding-top: 30px; }

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
