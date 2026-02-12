# css/css-overflow/scrollbar-gutter-dynamic-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-dynamic-003.html"
}
```

## style[0]

```css

  .line {
    display: flex;
  }

  .container {
    writing-mode: horizontal-tb;
    direction: ltr;
    block-size: 200px;
    inline-size: 200px;
    overflow: hidden;
    margin: 10px;
    background: deepskyblue;
  }

  .content {
    inline-size: 100%;
    block-size: 200%;
    background: lightsalmon;
  }

  .width-auto {
    scrollbar-width: auto;
  }

  .width-thin {
    scrollbar-width: thin;
  }

  .width-none {
    scrollbar-width: none;
  }

  .gutter-stable {
    scrollbar-gutter: stable;
  }

  .gutter-both {
    scrollbar-gutter: stable both-edges;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
