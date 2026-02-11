# css/css-break/flexbox/flex-item-content-overflow-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/flex-item-content-overflow-001-ref.html"
}
```

## style[0]

```css

  div {
    box-sizing: border-box;
  }
  .multicol {
    column-count: 2;
    column-gap: 0;
    column-fill: auto;
    inline-size: 300px;
    block-size: 100px;
    border: 10px solid purple;
  }
  .flexbox {
    display: block;
    inline-size: 130px;
    block-size: 70px;
    border: 10px solid black;
  }
  .item {
    block-size: 50px;
    border: 10px solid teal;
  }
  .grandchild {
    border: 10px solid orange;
    block-size: 140px;
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
