# css/css-view-transitions/support/iframe-scrollbar-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/support/iframe-scrollbar-child.html"
}
```

## style[0]

```css

      body.scrollable {
        width: 200lvw;
        height: 200lvh;
      }

      div {
        width: 200px;
        height: 200px;
        background-color: skyblue;
      }

      ::view-transition-new(*) {
        animation-duration: 30s;
        opacity: 0;
      }
      ::view-transition-old(*) {
        animation: unset;
        opacity: 1;
      }
      */
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
