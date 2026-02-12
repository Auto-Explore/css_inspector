# css/css-scrollbars/scrollbar-color-scheme-dynamic-3-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-scheme-dynamic-3-ref.html"
}
```

## style[0]

```css

  :root {
    color-scheme: light;
  }
  body {
    display: flex;
    flex-wrap: wrap;
    width: 200vw;
    height: 200vh;
  }

  .container {
    color-scheme: dark;
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
    background: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
