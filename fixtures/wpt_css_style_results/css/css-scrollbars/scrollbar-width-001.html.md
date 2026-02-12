# css/css-scrollbars/scrollbar-width-001.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-001.html"
}
```

## style[0]

```css

  /* Use scrollbar-gutter to reserve space for the scrollbar. */
  .container {
    scrollbar-gutter: stable;
    writing-mode: horizontal-tb;
    overflow: auto;
    height: 200px;
    width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 300px;
    width: 100%;
    background: lightsalmon;
  }

  /* writing directions */
  .ltr {
    direction: ltr;
  }

  .rtl {
    direction: rtl;
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
