# css/css-overflow/overflow-auto-scrollbar-gutter-intrinsic-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-auto-scrollbar-gutter-intrinsic-002-ref.html"
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
    scrollbar-gutter: stable both-edges;
    margin: 10px;
  }
  .hidden {
    overflow: hidden;
  }
  .scroll-x {
    overflow-x: scroll;
  }
  .scroll-y {
    overflow-y: scroll;
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
