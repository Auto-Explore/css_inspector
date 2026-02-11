# css/css-display/display-contents-pseudo-click-target.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-pseudo-click-target.html"
}
```

## style[0]

```css

  div {
    width: 100px;
    height: 100px;
    background-color: green;
    position: relative;
  }
  a {
    display: contents;

    &::after {
      content: "";
      position: absolute;
      inset: 0;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
