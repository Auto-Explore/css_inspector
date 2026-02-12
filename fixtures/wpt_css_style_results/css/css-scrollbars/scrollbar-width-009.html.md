# css/css-scrollbars/scrollbar-width-009.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-width-009.html"
}
```

## style[0]

```css

  :root {
    /* CSS scrollbar properties applied to the root element
       will be propagated to the viewport. */
    scrollbar-width: thin;
    overflow: visible;
  }

  body {
    /* overflow is propagated as well */
    overflow: scroll;
    /* but CSS scrollbar properties applied to the body are not propagated */
    scrollbar-width: none;
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
