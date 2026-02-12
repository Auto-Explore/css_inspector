# css/css-grid/grid-lanes/tentative/baseline/column-grid-lanes-container-baseline-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/column-grid-lanes-container-baseline-008-ref.html"
}
```

## style[0]

```css

    .container {
      display: flex;
      gap: 20px;
      border: 2px solid blue;
      width: 400px;
      padding: 10px;
      font-size: 16px;
      text-decoration: underline;
    }

    .multicolumn {
      column-count: 2;
      column-width: 100px;
      column-gap: 10px;
      border: 1px solid red;
      background: #f0f0f0;
      display: inline-block;
    }

    .multicolumn-item {
      background-color: transparent;
      padding: 8px;
      border: 4px solid navy;
      break-inside: avoid;
      margin-bottom: 10px;
      text-decoration: underline;
    }

    .multicolumn-item:nth-child(1) { height: 40px; font-size: 20px;color: lightcoral; display: flex; align-items: end; border-color: lightcoral;}
    .multicolumn-item:nth-child(3) { height: 80px; }
    .multicolumn-item:nth-child(2) { height: 60px; font-size: 22px;color: greenyellow; border-color: greenyellow;}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
