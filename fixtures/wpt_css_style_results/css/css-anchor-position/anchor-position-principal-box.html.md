# css/css-anchor-position/anchor-position-principal-box.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-principal-box.html"
}
```

## style[0]

```css

  body { margin-top: 0; }
  #outer {
    anchor-name: --anchor;
    display: contents;
  }
  #inner {
    anchor-name: --anchor;
  }
  #filler {
    height: 100px;
  }
  #anchored {
    position: absolute;
    top: anchor(--anchor top);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
