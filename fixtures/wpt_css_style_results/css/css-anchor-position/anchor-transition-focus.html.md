# css/css-anchor-position/anchor-transition-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-focus.html"
}
```

## style[0]

```css

  body:has(:focus) {
    --unused: foo;
  }
  #anchored {
    position: absolute;
    position-anchor: --foo;
    top: 0px;
    transition: top 0.1s steps(2, start);
  }
  :focus ~ #anchored {
    top: 40px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
