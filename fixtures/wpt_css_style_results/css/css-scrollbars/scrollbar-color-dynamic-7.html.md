# css/css-scrollbars/scrollbar-color-dynamic-7.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-dynamic-7.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: auto;
  }
  body {
    display: flex;
    flex-wrap: wrap;
    width: 200vw;
    height: 200vh;
  }

  .container {
    scrollbar-gutter: stable;
    overflow: auto;
    flex: 0 0;
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
