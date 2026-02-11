# css/css-anchor-position/anchored-child-transition-with-selection.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchored-child-transition-with-selection.html"
}
```

## style[0]

```css

  #container {
    position: absolute;
    right: anchor(right);
    --i: 1px;
  }
  #content {
    scale: 0;
    transition: scale 1000s step-end;
  }
  #content.show { scale: 1; }
  ::selection { color: var(--a); }
  #container.foo {}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
