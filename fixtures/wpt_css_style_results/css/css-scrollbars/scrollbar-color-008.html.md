# css/css-scrollbars/scrollbar-color-008.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-008.html"
}
```

## style[0]

```css

  body {
    scrollbar-color: yellow blue;
  }

  .container {
    scrollbar-gutter: stable;
    overflow: auto;
    height: 200px;
    min-width: 200px;
    margin: 1px;
    padding: 0px;
    border: none;
    background: deepskyblue;
  }

  .container::-webkit-scrollbar-corner {
    background-color: purple;
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
