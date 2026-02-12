# css/css-view-transitions/support/transition-in-empty-iframe-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/support/transition-in-empty-iframe-child.html"
}
```

## style[0]

```css

      ::view-transition-new(*) {
        animation: unset;
        opacity: 1;
      }
      ::view-transition-old(*) {
        animation-duration: 30s;
        opacity: 0;
      }

      div {
        width: 50vw;
        height: 50vh;
        background-color: limegreen;
        border: 1px solid black;
      }

      .hidden {
        display: none;
      }

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
