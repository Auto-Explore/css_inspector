# css/css-masking/clip-path/clip-path-fixed-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-fixed-nested.html"
}
```

## style[0]

```css

  body, html {
    margin: 0;
    padding: 0;
  }
  .outer-clip {
    height: 100vh;
    clip-path: inset(0 0 0 0);
    background: green;
  }
  .fixed {
    position: fixed;
  }
  .inner-clip {
    height: 50px;
    width: 50px;
    clip-path: inset(0 0 0 0);
  }
  .inner-clip-contents {
    height: 100px;
    width: 100px;
    background: purple;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
