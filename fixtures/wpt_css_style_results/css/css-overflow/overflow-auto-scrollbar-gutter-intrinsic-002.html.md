# css/css-overflow/overflow-auto-scrollbar-gutter-intrinsic-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-auto-scrollbar-gutter-intrinsic-002.html"
}
```

## style[0]

```css

  .line {
    display: flex;
  }
  .container {
    block-size: 50px;
    border: 5px solid black;
    overflow: auto;
    scrollbar-gutter: stable both-edges;
    margin: 10px;
  }
  .tall {
    /* trigger overflow */
    block-size: 5000px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
