# css/css-scrollbars/scrollbar-width-015.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-015.html"
}
```

## style[0]

```css

  .container {
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

  .container.auto {
    scrollbar-width: auto;
  }

  /* This is so that browsers that don't implement the WebKit prefix still pass the test */
  @supports not selector(::-webkit-scrollbar) {
    .container.auto {
      overflow: hidden;
    }
  }

  ::-webkit-scrollbar {
    display: none;
  }

  .container.thin {
    scrollbar-width: thin;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
