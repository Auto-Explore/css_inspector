# css/css-break/flexbox/flex-item-content-overflow-001b.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/flex-item-content-overflow-001b.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 2;
    column-gap: 0;
    column-fill: auto;
    inline-size: 280px;
    block-size: 80px;
    border: 10px solid purple;
  }
  .flexbox {
    display: flex;
    inline-size: 110px;
    block-size: 50px;
    border: 10px solid black;
  }
  .item {
    flex: 1;
    border: 10px solid teal;
  }
  .grandchild {
    border: 10px solid orange;
    block-size: 120px;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
