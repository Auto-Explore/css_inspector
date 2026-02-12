# css/css-scrollbars/scrollbar-width-paint-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-paint-001-ref.html"
}
```

## style[0]

```css

  body {
    display: flex;
    flex-wrap: wrap;
  }

  .container {
    scrollbar-gutter: stable;
    flex: 0 0;
    overflow: auto;
    height: 200px;
    min-width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .content {
    height: 300px;
    width: 300px;
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

  .content.plain {
    background: red;
  }

  .content.gradient {
    background: linear-gradient(135deg, red, blue);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
