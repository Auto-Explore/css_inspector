# css/css-scrollbars/scrollbar-width-002.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-002.html"
}
```

## style[0]

```css

  /* Use scrollbar-gutter to reserve space for the scrollbar. */
  .container {
    scrollbar-gutter: stable;
    overflow: auto;
    height: 200px;
    width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 100%;
    width: 300px;
    background: lightsalmon;
  }

  /* writing directions */
  .vertical-lr {
    writing-mode: vertical-lr;
  }

  .vertical-rl {
    writing-mode: vertical-rl;
  }

  .container.auto {
    scrollbar-width: auto;
  }

  .container.thin {
    scrollbar-width: thin;
  }

  .container.none {
    scrollbar-width: none;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
