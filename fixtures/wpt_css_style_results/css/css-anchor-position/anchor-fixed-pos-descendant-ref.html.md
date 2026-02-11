# css/css-anchor-position/anchor-fixed-pos-descendant-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-fixed-pos-descendant-ref.html"
}
```

## style[0]

```css

p {
  display: block;
  outline: 1px solid;
  height: 1lh;
  anchor-name: --foo;
  anchor-scope: --foo;
  overflow: hidden;
  resize: horizontal;
}

p > span {
  position: fixed;
  pointer-events: none;
  inset: anchor(--foo inside);
  right: anchor(--foo right, 0);
  left: 0;

  &::before {
    content: "";
    position: absolute;
    inset: 0;
    width: 10px;
    background-color: pink;
  }
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “pointer-events”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
