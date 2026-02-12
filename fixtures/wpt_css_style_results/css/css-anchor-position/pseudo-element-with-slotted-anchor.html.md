# css/css-anchor-position/pseudo-element-with-slotted-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/pseudo-element-with-slotted-anchor.html"
}
```

## style[0]

```css

    :host {
      display: block;
    }

    ::slotted(*) {
      display: block;
      anchor-name: --anchor;
      padding: 2rem;
      background: red;
      color: white;
      inline-size: fit-content;
    }

    :host::after {
      content: 'target';
      position: absolute;
      position-anchor: --anchor;
      position-area: bottom center;
      background: lightblue;
      padding: 1rem;
    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
