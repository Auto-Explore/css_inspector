# css/css-pseudo/highlight-cascade/highlight-cascade-008.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-008.html"
}
```

## style[0]

```css

  :root {
    --background-color: green;
    --decoration-color: yellow;
  }
  :root::selection {
    --background-color: cyan;
    --decoration-color: magenta;
  }
  div::selection {
    background-color: var(--background-color, red);
    text-decoration-line: underline;
    text-decoration-style: line;
    text-decoration-color: var(--decoration-color, red);
  }
  span {
    --background-color: purple;
  }
  span::selection {
    --background-color: blue;
    background-color: var(--background-color, red);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
