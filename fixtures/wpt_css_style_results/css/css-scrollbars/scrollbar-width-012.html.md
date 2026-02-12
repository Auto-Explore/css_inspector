# css/css-scrollbars/scrollbar-width-012.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-012.html"
}
```

## style[0]

```css

  :root {
    scrollbar-width: none;
  }

  :root::-webkit-scrollbar {
    width: 20px;
    background-color: lightgray;
  }

  :root,
  body {
    margin: 0;
    padding: 0;
  }

  #content {
    height: 10vh;
    width: 100%;
    background: lightsalmon;
  }

  #expander {
    /* force vertical scroll */
    height: 200vh;
    width: 300px;
    background: gray;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
