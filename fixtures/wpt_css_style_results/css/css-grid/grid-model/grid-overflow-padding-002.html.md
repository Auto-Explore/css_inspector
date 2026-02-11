# css/css-grid/grid-model/grid-overflow-padding-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-overflow-padding-002.html"
}
```

## style[0]

```css

    .grid {
       grid: 120px / 120px;
       width: 100px;
       height: 100px;
       box-sizing: border-box;
       padding: 10px 20px 20px 10px;
       overflow: auto;
    }
    .abspos {
    	position: absolute;
    	width: 10px;
    	height: 10px;
    	top: 140px;
    	left: 140px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
