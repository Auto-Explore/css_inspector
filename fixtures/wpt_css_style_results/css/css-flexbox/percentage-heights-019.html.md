# css/css-flexbox/percentage-heights-019.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-019.html"
}
```

## style[0]

```css

  .outer {
    display: flex;
  }

  .inner {
    display: flex;
    flex-direction: column;
    inline-size: 100px;
  }

  .item {
    overflow: hidden; /* This is essential to reproduce the bug. */
    min-block-size: 100px;
    background: red;
  }

  .child {
    block-size: 100%;
    background: green;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
