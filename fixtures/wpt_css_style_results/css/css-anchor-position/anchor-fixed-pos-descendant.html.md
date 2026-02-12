# css/css-anchor-position/anchor-fixed-pos-descendant.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-fixed-pos-descendant.html"
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
  left: anchor(--anchor right, 0);
  right: anchor(--foo right, 0);

  &::before {
    content: "";
    position: absolute;
    inset: 0;
    anchor-name: --anchor;
    width: 10px;
    background-color: pink;
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
