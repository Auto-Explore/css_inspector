# css/css-scrollbars/scrollbar-color-002.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-002.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: yellow blue;
  }

  :root::-webkit-scrollbar {
    display: none;
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
