# css/css-scrollbars/scrollbar-color-001.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-001.html"
}
```

## style[0]

```css

  :root {
    scrollbar-color: auto;
  }

  :root::-webkit-scrollbar {
    display: none;
  }

  /* This is so that browsers that don't implement the WebKit prefix still pass the test */
  @supports not selector(::-webkit-scrollbar) {
    :root {
      overflow: hidden;
    }
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
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
