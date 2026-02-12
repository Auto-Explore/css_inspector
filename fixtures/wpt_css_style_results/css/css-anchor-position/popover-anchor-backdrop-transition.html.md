# css/css-anchor-position/popover-anchor-backdrop-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/popover-anchor-backdrop-transition.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --anchor;
  }
  #popover {
    top: anchor(--anchor bottom);
  }
  ::backdrop {
    transition: opacity 1s step-end;
    opacity: 1;
  }
  @starting-style {
    ::backdrop {
      opacity: 0;
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
